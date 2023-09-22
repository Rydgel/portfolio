use axum::response::Html;
use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("404 lol"))
}
