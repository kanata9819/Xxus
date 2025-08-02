use dioxus::prelude::*;
use shared_types::CashFlow;

#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    pub flows: Vec<CashFlow>,
    pub target: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    rsx! {
        div { class: "list-container",
            h2 { "List of Items" }
            ul {
                for flow in props.flows {
                    if flow.flow == props.target {
                        li { "{flow.amount}" }
                        li { "{flow.name}" }
                        li { "{flow.date}" }
                    }
                }
            }
        }
    }
}
