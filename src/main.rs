mod routes;
mod app;
mod utils;
mod error;
mod appstate;
pub mod api;

use std::{net::SocketAddr, path::Path, time::Duration};

use axum::{
    body::Body,
    extract::{MatchedPath, connect_info::ConnectInfo},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use tokio::net::{UnixListener, UnixStream};
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::{ServeDir, ServeFile},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use log::{debug, error, info};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server,
};

use tower_service::Service;

#[cfg(unix)]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let prod = std::env::var("ENV").unwrap_or_else(|_| "dev".to_string()) == "prod";

    println!("Running in mode: {}", if prod { "Production" } else { "Development" });

    build_css().await;
    setup_tracing(prod);

    if prod {
        run_prod_server().await;
    } else {
        run_dev_server().await;
    }

    println!("Server shutdown");
}

#[cfg(not(unix))]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("Running in development");

    build_css().await;
    setup_tracing(false);

    run_dev_server().await;
    println!("Server shutdown");

}

pub async fn build_css() {

    if let Err(e) = utils::build_css().await {
        eprintln!("Error building CSS: {:?}", e);
    }
}



#[cfg(unix)]
async fn run_prod_server() {
    let socket_path = std::env::var("UNIX_SOCKET").expect("UNIX_SOCKET environment variable not set");
    let _ = std::fs::remove_file(&socket_path);

    let uds = UnixListener::bind(&socket_path).unwrap();
    println!("Listening on Unix socket: {}", socket_path);

    let app = app::app().await;
    let mut make_service = app.into_make_service_with_connect_info::<UdsConnectInfo>();

    
    loop {
        let (socket, _remote_addr) = uds.accept().await.unwrap();
        let tower_service = make_service.call(&socket).await.unwrap();

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);
            let hyper_service = hyper::service::service_fn(move |request: Request<hyper::body::Incoming>| {
                tower_service.clone().call(request)
            });

            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("Failed to serve connection: {:#}", err);
            }
        });
    }
}

async fn run_dev_server() {
    let port = 3000;
    let addr = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map(|listener| {
            println!("Listening on http://{}", listener.local_addr().unwrap());
            listener
        })
        .unwrap();

    axum::serve(addr, app::app().await.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(utils::shutdown())
        .await
        .unwrap();
}

fn setup_tracing(prod: bool) {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            format!(
                "{}={},tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME"),
                if prod { "error" } else { "debug" }
            )
            .into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
}

#[derive(Clone, Debug)]
struct UdsConnectInfo {
    peer_addr: std::sync::Arc<tokio::net::unix::SocketAddr>,
    peer_cred: tokio::net::unix::UCred,
}

impl axum::extract::connect_info::Connected<&UnixStream> for UdsConnectInfo {
    fn connect_info(target: &UnixStream) -> Self {
        let peer_addr = target.peer_addr().unwrap();
        let peer_cred = target.peer_cred().unwrap();
        Self {
            peer_addr: std::sync::Arc::new(peer_addr),
            peer_cred,
        }
    }
}
