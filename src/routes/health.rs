use axum::response::IntoResponse;
use serde_json::json;

pub async fn health_check_handler() -> impl IntoResponse {
    axum::Json(json!({ "message": "ok" }))
}
