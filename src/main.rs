use axum::body::StreamBody;
use axum::response::IntoResponse;
use axum::ServiceExt;
use axum::{routing::get, Router};
use hyper::header;
use jeromem_dev::routes::health_check_handler;
use jeromem_dev::routes::not_found_handler;
use jeromem_dev::telemetry::setup_tracing;
use tokio_util::io::ReaderStream;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route(
            "/robots.txt",
            get(|| serve_from_static_folder("robots.txt", "text/plain")),
        )
        .route(
            "/favicon.ico",
            get(|| serve_from_static_folder("favicon.ico", "image/x-icon")),
        )
        .route("/health", get(health_check_handler))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(setup_tracing())
        .fallback(not_found_handler);

    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let addr = "127.0.0.1:8080";
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn serve_from_static_folder(
    file_name: &'static str,
    content_type: &'static str,
) -> impl IntoResponse {
    let file = tokio::fs::File::open(format!("./static/{}", file_name))
        .await
        .unwrap();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let headers = [(header::CONTENT_TYPE, content_type)];

    (headers, body)
}
