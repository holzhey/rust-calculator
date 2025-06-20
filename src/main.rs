use axum::{
    Form, Router,
    routing::{get, post},
};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;

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
        @for number in 0..10 {
            button hx-post="/calc" hx-include="[name='result']" hx-target="[name='result']" hx-swap="outerHTML" name="digit" value=(number) { (number) };
        }
        input name="result" type="text" value="";
    }
}

#[derive(Deserialize, Debug)]
struct Operation {
    result: String,
    digit: String,
}

async fn calc(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.to_owned();
    let new_digit = operation.digit.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    html! {
         input name="result" type="text" value=(new_result);
    }
}

