use axum::Json;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::server::error::ServerError;
use crate::server::swagger::SwaggerExamples;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TemperatureResponse {
    #[schema(example = "Bedroom")]
    location: String,
    #[schema(example = "2")]
    sensor_id: String,
    #[schema(example = 24.1)]
    temperature: f64,
}

#[derive(Debug, Deserialize, IntoParams)]
pub(crate) struct LocationQuery {
    location: String,
}

#[utoipa::path(
    get,
    path = "/temperature",
    tag = "temperature",
    params(
        LocationQuery,
    ),
    responses(
        (
            status = 200,
            body = TemperatureResponse,
            content_type="application/json",
            description = "Report has been sent successful",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while generating and sending report",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn get_temperature(query: Query<LocationQuery>) -> impl IntoResponse {
    let location = &query.location;
    let sensor_id = get_sensor_by_location(location);
    let t_value = rand::random::<f64>();
    tracing::info!(temperature=t_value, location=location, "current sensor value");
    Json(TemperatureResponse {
        location: location.to_owned(),
        sensor_id: sensor_id.to_owned(),
        temperature: t_value,
    })
}

#[utoipa::path(
    get,
    path = "/temperature/{sensor_id}",
    tag = "temperature",
    params(
        (
            "sensor_id" = &str,
            description = "Sensor id to get temperature value",
            example = "1",
        ),
    ),
    responses(
        (
            status = 200,
            body = TemperatureResponse,
            content_type="application/json",
            description = "Report has been sent successful",
        ),
        (
            status = 400,
            body = ServerError,
            content_type="application/json",
            description = "Failed while generating and sending report",
            example = json!(ServerError::example(Some("internal error".to_string()))),
        ),
        (
            status = 503,
            body = ServerError,
            description = "Server does not available",
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn get_temperature_by_id(Path(sensor_id): Path<String>) -> impl IntoResponse {
    let location = get_location_by_sensor_id(&sensor_id);
    let t_value = rand::random::<f64>();
    tracing::info!(temperature=t_value, location=location, "current sensor value");
    Json(TemperatureResponse {
        location: location.to_owned(),
        sensor_id: sensor_id.to_owned(),
        temperature: t_value,
    })
}

fn get_location_by_sensor_id(sensor_id: &str) -> &str {
    match sensor_id {
        "1" => "Living Room",
        "2" => "Bedroom",
        "3" => "Kitchen",
        _ => "Unknown",
    }
}

fn get_sensor_by_location(location: &str) -> &str {
    match location {
        "Living Room" => "1",
        "Bedroom" => "2",
        "Kitchen" => "3",
        _ => "0",
    }
}
