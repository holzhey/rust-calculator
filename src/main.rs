use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        form {
            input type="button" value="1";
            input type="button" value="2";
            input type="text" name="output" value="";
            label for="output" { "Resultado" };
        }
    }
}
