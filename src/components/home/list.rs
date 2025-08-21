use dioxus::prelude::*;
use shared_types::CashFlow;

static CSS_PATH: Asset = asset!("/assets/components/home/list.css");

#[derive(Debug, Props, Clone, PartialEq)]
pub struct ListProps {
    pub flows: Signal<Vec<CashFlow>>,
    pub target: String,
    pub handle_delete_evt: EventHandler<i32>,
}

#[component]
pub fn List(props: ListProps) -> Element {
    // 選択中アイテム (id) を保持。None なら未選択
    let selected_id: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "list-container",
            div { class: "item-info",
                if !props.flows.read().iter().any(|f| f.flow == props.target) {
                    div { class: "empty-state",
                        div { class: "icon" }
                        div { class: "message", "データがありません" }
                        div { class: "sub-message", "新しい項目を追加してください" }
                    }
                } else {
                    ul { class: "items",
                        for flow in props.flows.read().iter().filter(|f| f.flow == props.target) {
                            {
                                let fid: i32 = flow.id;
                                let flow_type: String = flow.flow.clone();
                                let flow_name: String = flow.name.clone();
                                let flow_created: String = flow.created_at.clone();
                                let flow_amount: i32 = flow.amount;
                                let is_active: bool = selected_id().map(|id| id == fid).unwrap_or(false);
                                rsx! {
                                    li {
                                        class: if flow_type == "in" { if is_active { "income-item active" } else { "income-item" } } else { if is_active { "expense-item active" } else { "expense-item" } },
                                        onclick: {
                                            let mut selected_id_sig: Signal<Option<i32>> = selected_id.clone();
                                            move |_| {
                                                selected_id_sig
                                                    .set(
                                                        match selected_id_sig() {
                                                            Some(id) if id == fid => None,
                                                            _ => Some(fid),
                                                        },
                                                    );
                                            }
                                        },
                                        div { class: "item-details",
                                            div { class: "name", "{flow_name}" }
                                            div { class: "date", "{flow_created}" }
                                        }
                                        div { class: "amount", "{flow_amount}" }
                                        if is_active {
                                            div { class: "flex gap-2 ml-2 items-center",
                                                button {
                                                    class: "px-2 py-1 text-xs rounded bg-red-600 hover:bg-red-700 text-white",
                                                    onclick: move |_| {
                                                        (props.handle_delete_evt)(fid);
                                                    },
                                                    "削除"
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
        }
    }
}
