use dioxus::{prelude::*};
static CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn CalcHourlyWage() -> Element {
    let mut salary: Signal<String> = use_signal(|| String::new());
    let mut working_hours: Signal<String> = use_signal(|| String::new());
    let calculated_wage: Signal<String> = use_signal(|| String::new());

    rsx!{
        link { rel: "stylesheet", href: CSS_PATH }
        div {
            "時給計算"
            div { class: "mb-4",
                label { class: "m-2", "勤務時間" }
                input {
                    class: "border p-2 rounded",
                    r#type: "number",
                    value: working_hours,
                    oninput: move |e| working_hours.set(e.value()),
                }
            }

            div { class: "",
                label { "給与" }
                input {
                    class: "border p-2 rounded",
                    r#type: "number",
                    value: salary,
                    oninput: move |e| salary.set(e.value()),
                }
            }

            div {
                label { "計算結果" }
                div { "{calculated_wage()}" }
            }
        }
    }
}
