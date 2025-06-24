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
        .route("/calc", post(input))
        .route("/add", post(operation));
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
            button hx-post="/calc" hx-include="[name='output']" hx-target="[name='output']" name="action" value=(number) { (number) };
        }
        button hx-post="/add" hx-include="[name='output']" hx-target="[name='output']" name="action" value="+" { "+" };
        div name="output" {
            input name="result" type="text" value="";
            input name="accumulator" type="text" value="0";
       }
    }
}

#[derive(Deserialize, Debug)]
struct Operation {
    result: String,
    action: String,
    accumulator: String,
}

async fn input(Form(input): Form<Operation>) -> Markup {
    let current_result = input.result.to_owned();
    let current_acc = input.accumulator.to_owned();
    let new_digit = input.action.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    html! {
        input name="result" type="text" value=(new_result);
        input name="accumulator" type="text" value=(current_acc);
    }
}

async fn operation(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.parse::<i32>().unwrap();
    let current_acc = operation.accumulator.parse::<i32>().unwrap();
    let new_result = current_acc + current_result;
    html! {
        input name="result" type="text" value="";
        input name="accumulator" type="text" value=(new_result);
    }
}

