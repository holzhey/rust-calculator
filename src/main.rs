use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub mod handler;
pub mod view;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("trace,tower_http=trace"))
                .unwrap(),
        )
        .init();
    let app = Router::new()
        .route("/", get(handler::page))
        .route("/input", post(handler::input))
        .route("/operation", post(handler::operation))
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
