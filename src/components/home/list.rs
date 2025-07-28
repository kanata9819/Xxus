use dioxus::prelude::*;
use crate::structs::structs::CashFlows;

#[component]
pub fn List(props: CashFlows) -> Element {
    rsx! {
        div { class: "list-container",
            h2 { "List of Items" }
            ul {
                li { "Item 1" }
                li { "Item 2" }
                li { "Item 3" }
            }
        }
    }
}
