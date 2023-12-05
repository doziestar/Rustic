use axum::{Router, routing::get};
pub mod health;

pub fn status_routes() -> Router {
    Router::new()
        .route("/health", get(health::check_health))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::Request,
        http::StatusCode,
    };
    use hyper::body::to_bytes;
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new()
            .route("/health", get(health::check_health));

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        let (parts, body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::OK);

        let body_bytes = to_bytes(body).await.unwrap();
        let response_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(response_json, json!({ "status": "healthy" }));
    }
}
