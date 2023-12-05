use axum::response::Json;
use serde_json::json;

pub async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy" }))
}
