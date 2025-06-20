use axum::{
    Router,
    routing::{get, post},
};
use maud::{DOCTYPE, Markup, html};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/calc", post(calc));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
       (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@2.0.4" {};
        }
        button hx-post="/calc" hx-include="[name='result']" hx-target="[name='result']" hx-swap="outerHTML" name="digit" value="1" { "1" }
        input name="result" type="text" value="";
    }
}

async fn calc() -> Markup {
    html! {
         input name="result" type="text" value="1";
    }
}
