


pub mod index;
mod project;
pub mod people;
use project::*;

use askama::Template;
use axum::{response::{IntoResponse, Html}, routing::get, Router};
use tracing::error;
use crate::error::Error;

pub async fn project_router() -> Router {
    Router::new()
        .route("/exoskeleton", get(|| render_template(ExoskeletonTemplate)))
        .route("/lithography", get(|| render_template(LithographyTemplate)))
        .route("/plasma_jet", get(|| render_template(PlasmaJetTemplate)))
        .route("/energy_storage", get(|| render_template(EnergyStorageTemplate)))
        .route("/electrowetting", get(|| render_template(ElectrowettingTemplate)))
        .route("/drone", get(|| render_template(DroneTemplate)))
        .route("/biosensor", get(|| render_template(BiosensorTemplate)))
        .route("/analytics", get(|| render_template(AnalyticsTemplate)))
}


async fn render_template<T: Template>(template: T) -> impl IntoResponse {
    Html(template.render().unwrap())
}


pub fn render_failure(err: askama::Error) -> Error {
    error!("Template rendering failed: {}", err);
    Error::InternalServerError
}
