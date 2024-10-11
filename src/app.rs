use axum::{
    extract::MatchedPath, http::Request, response::Response, routing::get, Router
};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
    classify::ServerErrorsFailureClass,
};
use tracing::{info, error, Span};
use std::time::Duration;

use crate::routes::{self, index};

pub async fn app() -> Router {
 
 
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index::index))
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

    app
}
