use super::list::List;
use dioxus::prelude::*;
use tauri_sys::core::invoke;
use shared_types::CashFlow;

const CSS_PATH: Asset = asset!("/assets/components/home/home.css");

#[component]
pub fn Home() -> Element {
    let total_amount: Signal<i32> = use_signal(|| 0);
    let expense_amount: Signal<i32> = use_signal(|| 0);
    let income_amount: Signal<i32> = use_signal(|| 0);
    let cash_flows: Signal<Vec<CashFlow>> = use_signal(|| vec![]);

    use_future(move || async move {
        handle_load(cash_flows.clone()).await;
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "home-container",
            h1 { "Home" }
            div { class: "budget-container",
                div { class: "total-container",
                    h1 { "Total: {total_amount}" }
                }
                div { class: "income-expense-container",
                    div { class: "expense-container",
                        h1 { "Expense: {expense_amount}" }
                        List {
                            flows: cash_flows.read().to_vec(),
                            target: "Expense",
                        }
                    }
                    div { class: "income-container",
                        h1 { "Income: {income_amount}" }
                        List {
                            flows: cash_flows.read().to_vec(),
                            target: "Income",
                        }
                    }
                }
            }
            div { class: "input-container",
                input { class: "input", id: "input", placeholder: "amount" }
                div { class: "button-container",
                    button { class: "button", id: "submit-button", "submit" }
                }
            }
        }
    }
}

async fn list_cash_flows_or_empty() -> Vec<CashFlow> {
    invoke::<Vec<CashFlow>>("list_cash_flows", &()).await
}

async fn handle_load(mut cash_flows: Signal<Vec<CashFlow>>) {

    let result: bool = invoke::<bool>("init_db", &()).await;

    let flows: Vec<CashFlow> = list_cash_flows_or_empty().await;

    web_sys::console::log_1(&format!("init_db: {}", result).into());

    if flows.is_empty() {
        cash_flows.set(vec![]);
        return;
    }

    cash_flows.set(flows);
}
