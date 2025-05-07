use temperature_api::config::ServiceConfig;
use temperature_api::{logger, server};
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    logger::init_logger(config.logger())?;

    let listener = TcpListener::bind(config.server().address()).await?;
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let cors_layer = cors::CorsLayer::permissive();
    let app = server::init_server().layer(trace_layer).layer(cors_layer);
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    }

    Ok(())
}
