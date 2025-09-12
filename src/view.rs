use maud::{DOCTYPE, Markup, html};

fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@2.0.4" {};
        }
    }
}

pub fn page() -> Markup {
    html! {
        (header())
        @for number in 0..=9 {
            (input(&number.to_string()))
        }
        (operation("+"))
        (output("0", "0"))
    }
}

pub fn input(value: &str) -> Markup {
    button("/input", value)
}

pub fn operation(action: &str) -> Markup {
    button("/operation", action)
}

pub fn output(result: &str, accumulator: &str) -> Markup {
    html! {
        div name="output" {
            input name="result" type="text" value=(result);
            input name="accumulator" type="text" value=(accumulator);
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
