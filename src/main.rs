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
        .route("/calc", post(calc))
        .route("/add", post(add));
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
        @for number in 0..=9 {
            button hx-post="/calc" hx-include="[name='output']" hx-target="[name='output']" name="digit" value=(number) { (number) };
        }
        button hx-post="/add" hx-include="[name='output']" hx-target="[name='output']" value="+" { "+" };
        div name="output" {
            input name="result" type="text" value="";
            input name="accumulator" type="hidden" value="";
       }
    }
}

#[derive(Deserialize, Debug)]
struct Operation {
    result: String,
    digit: String,
    accumulator: String,
}

async fn calc(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.to_owned();
    let current_acc = operation.accumulator.to_owned();
    let new_digit = operation.digit.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    html! {
        input name="result" type="text" value=(new_result);
        input name="accumulator" type="hidden" value=(current_acc);
    }
}

async fn add(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.to_owned();
    let current_acc = operation.accumulator.to_owned();
    let new_digit = operation.digit.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    html! {
        input name="result" type="text" value="";
        input name="accumulator" type="hidden" value=(current_acc);
    }
}

