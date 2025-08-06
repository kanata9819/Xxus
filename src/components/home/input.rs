use dioxus::prelude::*;
use serde_json;
use shared_types::AddCashFlowProps;
use tauri_sys::core::invoke;

#[component]
pub fn Inputs(
    total: Signal<i32>,
    name: Signal<String>,
    amount: Signal<i32>,
    flow_type: &'static str,
    parent_need_refresh: Signal<bool>
) -> Element {
    let mut input_name: Signal<String> = use_signal(|| String::new());
    let mut input_amount: Signal<i32> = use_signal(|| 0);

    rsx! {
        div { class: "input-form",
            div { class: "form-group",
                label { "項目名" }
                input {
                    class: "input-field",
                    placeholder: "項目名を入力",
                    oninput: move |e| {
                        input_name.set(e.value());
                    },
                    value: input_name,
                }
            }
            div { class: "form-group",
                label { "金額" }
                input {
                    class: "input-field",
                    placeholder: "金額を入力",
                    r#type: "number",
                    step: "1",
                    oninput: move |e| {
                        let val = e.value();
                        if let Ok(v) = val.parse::<i32>() {
                            input_amount.set(v);
                        }
                    },
                    value: input_amount,
                }
            }
            div { class: "button-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_: MouseEvent| async move {
                        {
                            handle_submit(input_name(), input_amount(), flow_type, parent_need_refresh)
                                .await
                        };
                    },
                    "追加"
                }
            }
        }
    }
}

async fn handle_submit(
    input_name: String,
    input_amount: i32,
    flow_type: &str,
    mut parent_need_refresh: Signal<bool>
) {
    if input_name.is_empty() {
        return;
    } else if input_amount == 0 {
        return;
    }

    let props: AddCashFlowProps = AddCashFlowProps {
        name: (input_name),
        amount: (input_amount),
        flow_type: (flow_type.to_string()),
    };

    let result: bool = invoke::<bool>("add_cash_flow", &serde_json::json!({"props": props})).await;

    if result {
        parent_need_refresh.set(true);
    } else {
        web_sys::console::log_1(&format!("NG").into());
    }
}
