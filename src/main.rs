use axum::{
    Router,
    routing::{get, post},
};
use handler::{input_handler, operation_handler, page_handler};

pub mod handler;
pub mod view;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(page_handler))
        .route("/input", post(input_handler))
        .route("/operation", post(operation_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
