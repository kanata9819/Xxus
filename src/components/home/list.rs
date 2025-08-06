use dioxus::{prelude::*};
use shared_types::CashFlow;

static CSS_PATH: Asset = asset!("/assets/components/home/list.css");

#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    pub flows: Vec<CashFlow>,
    pub target: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let filtered_flows: Vec<CashFlow> = props.flows.into_iter()
        .filter(|flow| flow.flow == props.target)
        .collect();

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "list-container",
            div { class: "item-info",
                if filtered_flows.is_empty() {
                    div { class: "empty-state",
                        div { class: "icon" }
                        div { class: "message", "データがありません" }
                        div { class: "sub-message", "新しい項目を追加してください" }
                    }
                } else {
                    ul { class: "items",
                        for flow in filtered_flows {
                            li { 
                                class: if flow.flow == "in" { "income-item" } else { "expense-item" },
                                div { class: "item-details",
                                    div { class: "name", "{flow.name}" }
                                    div { class: "date", "{flow.date}" }
                                }
                                div { class: "amount", "{flow.amount}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
