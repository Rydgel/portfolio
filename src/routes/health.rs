use axum::response::IntoResponse;
use axum::response::Json;
use serde_json::json;

pub async fn health_check_handler() -> impl IntoResponse {
    Json(json!({ "message": "ok" }))
}
