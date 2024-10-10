use axum::{routing::get, Router};

mod pages;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(pages::home::render))
        .route("/iss", get(pages::iss::render))
        .route("/weaviate", get(pages::weaviate::render))
        .route("/version", get(pages::version::render));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
