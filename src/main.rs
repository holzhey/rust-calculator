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
        .route("/input", post(input))
        .route("/operation", post(operation));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
        (header())
        @for number in 0..=9 {
            (button("/input", &number.to_string()))
        }
        (button("/operation", "+"))
        (output("", "0"))
    }
}

#[derive(Deserialize, Debug)]
struct Operation {
    result: String,
    action: String,
    accumulator: String,
}

fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@2.0.4" {};
        }
    }
}

fn button(target: &str, value: &str) -> Markup {
    html! {
        button
            hx-post=(target)
            hx-include="[name='output']"
            hx-target="[name='output']"
            name="action" value=(value) { (value) };
    }
}

fn output(result: &str, accumulator: &str) -> Markup {
    html! {
        div name="output" {
            input name="result" type="text" value=(result);
            input name="accumulator" type="text" value=(accumulator);
        }
    }
}

async fn input(Form(input): Form<Operation>) -> Markup {
    let current_result = input.result.to_owned();
    let current_acc = input.accumulator.to_owned();
    let new_digit = input.action.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    output(&new_result, &current_acc.to_string())
}

async fn operation(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.parse::<i32>().unwrap();
    let current_acc = operation.accumulator.parse::<i32>().unwrap();
    let new_result = current_acc + current_result;
    output("", &new_result.to_string())
}

