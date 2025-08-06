use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet};
use shared_types::{CashFlow};
use tauri_sys::core::invoke;
use wasm_bindgen_futures::spawn_local;
use super::input::Inputs;
use super::list::List;

const CSS_PATH: Asset = asset!("/assets/components/home/home.css");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HomeSignals {
    pub name: Signal<String>,
    pub total: Signal<i32>,                    // 画面値
    pub expense: Signal<i32>,                  // 画面値
    pub income: Signal<i32>,                   // 画面値
    pub vec_cash_flows: Signal<Vec<CashFlow>>, // DB値(配列)
}

#[component]
pub fn Home() -> Element {
    let item_name: Signal<String> = use_signal(|| String::new());
    let total_amount: Signal<i32> = use_signal(|| 0);
    let expense_amount: Signal<i32> = use_signal(|| 0);
    let income_amount: Signal<i32> = use_signal(|| 0);
    let cash_flows: Signal<Vec<CashFlow>> = use_signal(|| vec![]);

    let mut disp_ex_input: Signal<bool> = use_signal(|| false);
    let mut disp_in_input: Signal<bool> = use_signal(|| false);
    let mut need_refresh: Signal<bool> = use_signal(|| false);

    let home_strc: HomeSignals = HomeSignals {
        name: (item_name),
        total: (total_amount),
        expense: (expense_amount),
        income: (income_amount),
        vec_cash_flows: (cash_flows),
    };

    use_future(move || async move {
        handle_load(home_strc.clone()).await;
    });

    use_effect(move || {
        if need_refresh() {
            initialize(home_strc);

            spawn_local(async move {
                handle_load(home_strc.clone()).await;
            });

            need_refresh.set(false);
        }
    });

    rsx! {
        MaterialIconStylesheet {}
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "home-container",
            MaterialIcon { name: "home", size: Some(24) }
            div { class: "budget-container",
                div { class: "total-container",
                    h1 { "{total_amount}" }
                }
                div { class: "income-expense-container",
                    div { class: "expense-container",
                        h1 { "Expense" }
                        List { flows: cash_flows.read().to_vec(), target: "ex" }
                        button {
                            class: "add-button",
                            onclick: move |_: MouseEvent| {
                                disp_ex_input.set(!(disp_ex_input)());
                            },
                            if !disp_ex_input() {
                                "+"
                            } else {
                                "×"
                            }
                        }
                        if disp_ex_input() {
                            div { class: "input-container",
                                Inputs {
                                    total: home_strc.total,
                                    name: home_strc.name,
                                    amount: home_strc.expense,
                                    flow_type: "ex",
                                    parent_need_refresh: need_refresh,
                                }
                            }
                        }
                    }
                    div { class: "income-container",
                        h1 { "Income" }
                        List { flows: cash_flows.read().to_vec(), target: "in" }
                        button {
                            class: "add-button",
                            onclick: move |_: MouseEvent| {
                                disp_in_input.set(!(disp_in_input)());
                            },
                            if !disp_in_input() {
                                "+"
                            } else {
                                "×"
                            }
                        }
                        if disp_in_input() {
                            div { class: "input-container",
                                Inputs {
                                    total: home_strc.total,
                                    name: home_strc.name,
                                    amount: home_strc.income,
                                    flow_type: "in",
                                    parent_need_refresh: need_refresh,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn initialize(mut strc: HomeSignals) {
    strc.name.set("".to_string());
    strc.income.set(0);
    strc.expense.set(0);
    strc.total.set(0);
    strc.vec_cash_flows.set(vec![]);
}

async fn handle_load(mut home_strc: HomeSignals) {
    invoke::<bool>("init_db", &()).await;

    let flows: Vec<CashFlow> = list_cash_flows_or_empty().await;

    if flows.is_empty() {
        home_strc.vec_cash_flows.set(vec![]);
        return;
    }

    home_strc.vec_cash_flows.set(flows);

    set_amount(home_strc);
}

async fn list_cash_flows_or_empty() -> Vec<CashFlow> {
    invoke::<Vec<CashFlow>>("list_cash_flows", &()).await
}

fn set_amount(mut home_strc: HomeSignals) {
    let mut total: i32 = 0;
    let mut expense: i32 = 0;
    let mut income: i32 = 0;

    for data in home_strc.vec_cash_flows.iter() {
        if data.flow == "ex" {
            total -= data.amount;
            expense -= data.amount;
        } else if data.flow == "in" {
            total += data.amount;
            income += data.amount;
        }
    }

    home_strc.total.set(total);
    home_strc.expense.set(expense);
    home_strc.income.set(income);
}
