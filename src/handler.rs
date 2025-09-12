use axum::Form;
use maud::Markup;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Operation {
    result: u32,
    action: String,
    accumulator: u32,
}

pub async fn page() -> Markup {
    crate::view::page()
}

pub async fn input(Form(input): Form<Operation>) -> Markup {
    let current_result = input.result;
    let current_acc = input.accumulator;
    let new_digit = input.action;
    let new_result = format!("{current_result}{new_digit}");
    crate::view::output(&new_result, &current_acc.to_string())
}

pub async fn operation(Form(operation): Form<Operation>) -> Markup {
    let current_result = operation.result;
    let current_acc = operation.accumulator;
    let new_result = current_acc + current_result;
    crate::view::output("0", &new_result.to_string())
}
