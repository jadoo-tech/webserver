
mod routes;

use std::time::Duration;

use askama::Template;
use axum::{
    body::Body, extract::MatchedPath, http::{Request, StatusCode}, response::{Html, IntoResponse, Response}, routing::get, Router
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

    build_css().await;

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

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/always-fails", get(always_fails))
        .nest("/project", routes::project_router().await)
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.svg"))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, span: &Span| {
                //
                info!("Request");
            })
            .on_response(|response: &Response, latency: Duration, span: &Span| {
                info!("{:?} - {:?}", response.status(), latency);
            })
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    error!("Error: {:?}", _error);
                },
            ),
    );

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
    axum::serve(listener, app).with_graceful_shutdown(shutdown()).await.unwrap();
    println!("Server shutdown");

}

async fn shutdown() {
    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutting down");
}

async fn always_fails() -> Result<(), Error> {
    info!("Always fails");
    Err(Error::InternalServerError)
}

pub enum Error {
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(match self {
                Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            })
            .body(Body::empty())
            .unwrap()
    }
}

async fn index() -> impl IntoResponse {
    let hello = HelloTemplate { name: "World" };
    hello.render().map_err(render_failure).map(|u| Html(u))
}

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

pub fn render_failure(err: askama::Error) -> Error {
    error!("Template rendering failed: {}", err);
    Error::InternalServerError
}

async fn build_css() {
    let res = Command::new("tailwind")
        .args([
            "-i",
            "./src/styles.css",
            "-o",
            "./static/styles.css",
            "--minify",
        ])
        .status()
        .await;

    match res {
        Ok(status) => {
            if !status.success() {
                error!("CSS build failed");
            } else {
                println!("CSS build successful");
            }
        }
        Err(e) => {
            error!("CSS build failed: {}", e);
        }
    }
}