
mod routes;
mod app;
mod utils;
mod error;
mod appstate;
mod api;

use std::{net::SocketAddr, path::Path, time::Duration};

use askama::Template;
use axum::{
    body::Body, extract::MatchedPath, http::{Request, StatusCode}, response::{Html, IntoResponse, Response}, routing::get, Router, ServiceExt
};

use tokio::process::Command;
use tower_http::{classify::ServerErrorsFailureClass, services::{ServeDir, ServeFile}, trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer}};
use log::{debug, error, info};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let prod = if std::env::var("ENV").unwrap_or("dev".to_string()) == "prod".to_string() {
        true
    } else {
        false
    };

    println!("Running in mode: {}", if prod { "Production" } else { "Development" });

    if let Err(e) = utils::build_css().await {
        eprintln!("Error building CSS: {:?}", e);
    }

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

    // run our app with hyper, listening globally on port 3000
    let mut port = 3000;
    if prod {
        if let Ok(port_str) = std::env::var("PORT") {
            if let Ok(port_num) = port_str.parse::<usize>() {
                port = port_num;
            }
        } else {
            error!("Running in prod but PORT environment variable not set");
        }
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app::app().await.into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(utils::shutdown()).await.unwrap();
    println!("Server shutdown");
}
