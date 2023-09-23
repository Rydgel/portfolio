use axum::response::{Html, IntoResponse};

pub async fn index_handler() -> impl IntoResponse {
    Html("<div>hello</div>")
}
