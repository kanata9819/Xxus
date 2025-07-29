use dioxus::prelude::*;
use super::list::List;
use crate::structs::structs::CashFlows;
use crate::structs::structs::CashFlow;
use crate::enums::enum_global::FlowType;

const CSS_PATH: Asset = asset!("/assets/components/home/home.css");

#[component]
pub fn Home() -> Element {
    let total_amount: Signal<i32> = use_signal(|| 1000);
    let expense_amount: Signal<i32> = use_signal(|| 100);
    let income_amount: Signal<i32> = use_signal(|| 100);
    let cash_flows = use_signal(|| CashFlows { flows: vec![] });

    let () = use_hook(|| {
        handle_load(cash_flows);
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
                        List { flows: cash_flows.read().flows.clone() }
                    }
                    div { class: "income-container",
                        h1 { "Income: {income_amount}" }
                        List { flows: cash_flows.read().flows.clone() }
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

fn handle_load(
    mut cash_flows: Signal<CashFlows>,
) {
    // TODO: Load data from database
    cash_flows.set(CashFlows {
        flows: vec![
            CashFlow {
                id: 1,
                amount: 300,
                name: "Expense 1".to_string(),
                flow: FlowType::Expense,
            },
            CashFlow {
                id: 2,
                amount: 700,
                name: "Income 1".to_string(),
                flow: FlowType::Income,
            },
        ],
    });
}
