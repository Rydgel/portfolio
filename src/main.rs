use axum::{routing::get, Router};
use jeromem_dev::routes::health_check_handler;
use jeromem_dev::telemetry::setup_tracing;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .layer(setup_tracing());

    let addr = "127.0.0.1:8080";
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
