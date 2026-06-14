use axum::{
    Router,
    middleware as axum_mw,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub mod handlers;

pub async fn build_router() -> anyhow::Result<Router> {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/v1/meters", get(handlers::list_meters))
        .route("/api/v1/meters/:id", get(handlers::get_meter))
        .route("/api/v1/tariffs", get(handlers::list_tariffs))
        .route("/api/v1/readings", post(handlers::submit_reading))
        .route("/api/v1/settle", post(handlers::settle_account))
        .route("/metrics", get(handlers::metrics_handler))
        .layer(axum_mw::from_fn(crate::api::middleware::rate_limit_layer))
        .layer(cors);

    Ok(app)
}
