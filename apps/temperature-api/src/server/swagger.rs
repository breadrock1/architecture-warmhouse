use utoipa::OpenApi;

use crate::server::routes::*;

#[derive(OpenApi)]
#[openapi(
    info(
        description = "Temperature-API Service."
    ),
    tags(
        (
            name = "temperature",
            description = "Temperature API"
        ),
    ),
    paths(
        get_temperature,
        get_temperature_by_id,
    ),
    components(),
)]
pub(super) struct ApiDoc;

pub trait SwaggerExamples {
    type Example: serde::Serialize;

    fn example(value: Option<String>) -> Self::Example;
}
