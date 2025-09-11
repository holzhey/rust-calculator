use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

pub mod handler;
pub mod view;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler::page))
        .route("/input", post(handler::input))
        .route("/operation", post(handler::operation));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
