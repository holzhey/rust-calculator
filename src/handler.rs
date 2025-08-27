use axum::Form;
use maud::Markup;
use serde::Deserialize;

use crate::view::{output, page};

#[derive(Deserialize, Debug)]
pub struct Operation {
    result: String,
    action: String,
    accumulator: String,
}

pub async fn page_handler() -> Markup {
    page()
}

pub async fn input_handler(Form(input): Form<Operation>) -> Markup {
    let current_result = input.result.to_owned();
    let current_acc = input.accumulator.to_owned();
    let new_digit = input.action.to_owned();
    let new_result = format!("{current_result}{new_digit}");
    output(&new_result, &current_acc.to_string())
}

pub async fn operation_handler(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result.parse::<i32>().unwrap();
    let current_acc = operation.accumulator.parse::<i32>().unwrap();
    let new_result = current_acc + current_result;
    output("0", &new_result.to_string())
}
