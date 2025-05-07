use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::server::swagger::SwaggerExamples;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("reporter module error: {0}")]
    Reporter(String),
    #[error("service unavailable")]
    ServiceUnavailable,
}

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::InternalError(msg) => (msg.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::Reporter(err) => (err.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::ServiceUnavailable => (
                "service unavailable".to_string(),
                StatusCode::SERVICE_UNAVAILABLE,
            ),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse { message: msg }).into_response();
        *resp.status_mut() = status;
        resp
    }
}

impl SwaggerExamples for ServerError {
    type Example = Self;

    fn example(value: Option<String>) -> Self::Example {
        match value {
            None => ServerError::ServiceUnavailable,
            Some(msg) => ServerError::InternalError(msg),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Success {
    status: u16,
    message: String,
}

impl SwaggerExamples for Success {
    type Example = Self;

    fn example(value: Option<String>) -> Self::Example {
        Success {
            status: 200,
            message: value.unwrap_or("Ok".to_string()),
        }
    }
}
