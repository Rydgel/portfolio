use serde::Serialize;

#[derive(Serialize)]
pub struct HealthPayload {
    message: String,
}

pub async fn health_check_handler() -> axum::response::Json<HealthPayload> {
    axum::Json(HealthPayload {
        message: "ok".into(),
    })
}
