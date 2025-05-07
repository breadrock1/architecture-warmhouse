pub mod config;
mod error;
mod routes;
mod swagger;

use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::server::swagger::ApiDoc;

const OPENAPI_PATH: &str = "/api-docs/openapi.json";
const OPENAPI_URL_PATH: &str = "/rapidoc";

#[derive(Default)]
struct ServerApp;

pub fn init_server() -> Router {
    let app_arc = Arc::new(ServerApp::default());
    Router::new()
        .merge(RapiDoc::with_openapi(OPENAPI_PATH, ApiDoc::openapi()).path(OPENAPI_URL_PATH))
        .route("/temperature", get(routes::get_temperature))
        .route("/temperature/{sensor_id}", get(routes::get_temperature_by_id))
        .with_state(app_arc)
}
