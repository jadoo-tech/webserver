use axum::{
    extract::{ConnectInfo, MatchedPath}, http::Request, response::{IntoResponse, Response}, routing::{any, get}, Extension, Router
};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
    classify::ServerErrorsFailureClass,
};
use tracing::{info, error, Span};
use std::{net::SocketAddr, sync::Arc, time::Duration};

use crate::{appstate::AppState, routes::{self, index}, api::{self, api_router}};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum_extra::{headers, TypedHeader};
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn app() -> Router {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index::index))
        .nest("/api", api::api_router().await)
        .nest("/project", routes::project_router().await)
        .route("/people", get(routes::people::people))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.svg"))
        .nest_service("/static", ServeDir::new("static"))
        .route("/ws", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, span: &Span| {
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
        )
        .layer(Extension(AppState::new()));

    app
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    Extension(state): Extension<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: AppState) {
    // Send the version hash to the client
    if let Err(e) = socket.send(Message::Text(state.version_hash.clone())).await {
        println!("Error sending version hash to {who}: {:?}", e);
        return;
    }

    println!("Sent version hash to {who}");

    // Keep the connection open and handle any incoming messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Close(_)) => break,
            Err(e) => {
                println!("Error receiving message from {who}: {:?}", e);
                break;
            }
            _ => {} // Ignore other messages
        }
    }

    println!("WebSocket connection closed for {who}");
}
