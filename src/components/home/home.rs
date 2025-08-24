use super::input::Inputs;
use super::list::List;
use dioxus::prelude::*;
// use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet}; // リリース問題切り分けのため無効化
use serde_json;
use shared_types::CashFlow;
use tauri_sys::core::invoke;
use wasm_bindgen_futures::spawn_local;

static CSS_PATH: Asset = asset!("assets/styles.css");

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
        if *need_refresh.read() == true {
            initialize(home_strc);

            spawn_local(async move {
                handle_load(home_strc.clone()).await;
            });

            need_refresh.set(false);
        }
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        // MaterialIconStylesheet {}
        div { class: "home-container flex flex-col gap-4 w-[85vw] max-w-[100vw] mx-auto mt-4",
            span { class: "text-xl mr-1", "🏠" }
            div { class: "budget-container",
                div { class: "total-container modal-panel-dark rounded-lg border border-gray-200 shadow-xl p-4 flex items-center justify-between",
                    h1 { class: "section-title", "￥{total_amount}" }
                }
                div { class: "income-expense-container flex flex-col lg:flex-row gap-4 min-h-0 h-[65vh]",
                    div { class: "expense-container modal-panel-dark rounded-lg border border-gray-200 shadow p-4 flex flex-col gap-3 flex-1 min-h-0 h-full",
                        h1 { class: "font-semibold", "Expense" }
                        div { class: "overflow-y-auto flex-1 pr-1 min-h-0",
                            List {
                                flows: cash_flows,
                                target: "ex",
                                handle_delete_evt: {
                                    move |id: i32| {
                                        spawn_local(async move {
                                            handle_delete(id, home_strc.clone()).await;
                                        });
                                    }
                                },
                            }
                        }
                        button {
                            class: "btn-primary self-end",
                            onclick: move |_: MouseEvent| {
                                disp_ex_input.set(!(disp_ex_input)());
                            },
                            if !disp_ex_input() {
                                "+"
                            } else {
                                "×"
                            }
                        }
                    }
                    div { class: "income-container modal-panel-dark rounded-lg border border-gray-200 shadow p-4 flex flex-col gap-3 flex-1 min-h-0 h-full",
                        h1 { class: "font-semibold", "Income" }
                        div { class: "overflow-y-auto flex-1 pr-1 min-h-0",
                            List {
                                flows: cash_flows,
                                target: "in",
                                handle_delete_evt: {
                                    move |id: i32| {
                                        spawn_local(async move {
                                            handle_delete(id, home_strc.clone()).await;
                                        });
                                    }
                                },
                            }
                        }
                        button {
                            class: "btn-primary self-end",
                            onclick: move |_: MouseEvent| {
                                disp_in_input.set(!(disp_in_input)());
                            },
                            if !disp_in_input() {
                                "+"
                            } else {
                                "×"
                            }
                        }
                    }
                }
                // Overlays
                if disp_ex_input() {
                    Fragment {
                        div {
                            class: "fixed inset-0 bg-black/40 backdrop-blur-[1px] z-40",
                            onclick: move |_| disp_ex_input.set(false),
                        }
                        div { class: "fixed inset-0 z-50 flex items-center justify-center",
                            div { class: "pointer-events-auto modal-panel-dark rounded-lg shadow-xl w-[90vw] max-w-[640px] max-h-[85vh] overflow-hidden border border-gray-200",
                                div { class: "flex items-center justify-between px-4 py-2 border-b",
                                    h3 { class: "font-semibold", "Expense の追加" }
                                    button {
                                        class: "text-gray-500 hover:text-gray-700 text-xl leading-none",
                                        onclick: move |_| disp_ex_input.set(false),
                                        "×"
                                    }
                                }
                                div { class: "p-4 overflow-y-auto max-h-[75vh]",
                                    Inputs {
                                        total: home_strc.total,
                                        name: home_strc.name,
                                        amount: home_strc.expense,
                                        flow_type: "ex",
                                        parent_need_refresh: need_refresh,
                                        disp_input: disp_ex_input,
                                    }
                                }
                            }
                        }
                    }
                }
                if disp_in_input() {
                    Fragment {
                        div {
                            class: "fixed inset-0 bg-black/40 backdrop-blur-[1px] z-40",
                            onclick: move |_| disp_in_input.set(false),
                        }
                        div { class: "fixed inset-0 z-50 flex items-center justify-center",
                            div { class: "pointer-events-auto modal-panel-dark rounded-lg shadow-xl w-[90vw] max-w-[640px] max-h-[85vh] overflow-hidden border border-gray-200",
                                div { class: "flex items-center justify-between px-4 py-2 border-b",
                                    h3 { class: "font-semibold", "Income の追加" }
                                    button {
                                        class: "text-gray-500 hover:text-gray-700 text-xl leading-none",
                                        onclick: move |_| disp_in_input.set(false),
                                        "×"
                                    }
                                }
                                div { class: "p-4 overflow-y-auto max-h-[75vh]",
                                    Inputs {
                                        total: home_strc.total,
                                        name: home_strc.name,
                                        amount: home_strc.income,
                                        flow_type: "in",
                                        parent_need_refresh: need_refresh,
                                        disp_input: disp_in_input,
                                    }
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

async fn handle_delete(id: i32, home_strc_props: HomeSignals) {
    let result: bool = delete_specific_data(id).await;

    if result {
        handle_load(home_strc_props).await;
    }
}

async fn delete_specific_data(id: i32) -> bool {
    invoke::<bool>("delete_specific_data", &serde_json::json!({ "id": id })).await
}
