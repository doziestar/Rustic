use axum::{Router, routing::get};
pub mod health;

pub fn status_routes() -> Router {
    Router::new()
        .route("/health", get(health::check_health))
}