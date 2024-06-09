use tokio::net::TcpListener;
use axum::{routing::get, Router};

#[tokio::main(flavor="multi_thread", worker_threads=4)]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let _ = axum::serve(listener, app);
    Ok(())
}

async fn health_check() -> &'static str {
    "Health OK"
}