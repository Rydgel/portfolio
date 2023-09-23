use axum::body::StreamBody;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use axum_extra::routing::RouterExt;
use hyper::header;
use jeromem_dev::routes::not_found_handler;
use jeromem_dev::routes::{health_check_handler, index_handler};
use jeromem_dev::telemetry::setup_tracing;
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_http::compression::predicate::SizeAbove;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route(
            "/robots.txt",
            get(|| serve_from_static_folder("robots.txt", "text/plain")),
        )
        .route(
            "/favicon.ico",
            get(|| serve_from_static_folder("favicon.ico", "image/x-icon")),
        )
        .route_with_tsr("/health", get(health_check_handler))
        .route("/", get(index_handler))
        .layer(
            ServiceBuilder::new()
                .layer(setup_tracing())
                .layer(CompressionLayer::new().compress_when(SizeAbove::new(0))),
        )
        .fallback(not_found_handler);

    let addr = "127.0.0.1:8080";
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
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
