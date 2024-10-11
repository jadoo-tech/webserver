use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}};


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

