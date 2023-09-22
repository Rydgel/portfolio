use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use jeromem_dev::routes::health_check_handler;
use jeromem_dev::telemetry::setup_tracing;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/health", get(health_check_handler))
        .layer(setup_tracing());

    let addr = "127.0.0.1:8080";
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
