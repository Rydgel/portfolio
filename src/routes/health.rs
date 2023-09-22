use serde_json::{json, Value};

pub async fn health_check_handler() -> axum::response::Json<Value> {
    axum::Json(json!({ "message": "ok" }))
}
