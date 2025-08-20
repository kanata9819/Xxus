use dioxus::prelude::*;
use shared_types::CashFlow;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("/assets/components/home/list.css");

#[derive(Debug, Props, Clone, PartialEq)]
pub struct ListProps {
    pub flows: Vec<CashFlow>,
    pub target: String,
}

#[component]
pub fn List(props: ListProps) -> Element {

    let flows_from_props = props.flows.clone();
    // 選択中アイテム (id) を保持。None なら未選択
    let mut selected_id: Signal<Option<i32>> = use_signal(|| None);

    // flows をローカルにクローン (イベントクロージャ 'static 制約を満たすため)
    let mut flows_sig: Signal<Vec<CashFlow>> = use_signal(|| flows_from_props.clone());

    // use_effect だと初回のみなので、毎レンダーで差分同期
    if flows_sig.read().as_slice() != flows_from_props.as_slice() {
        flows_sig.set(flows_from_props.clone());
    }

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "list-container",
            div { class: "item-info",
                if !flows_sig.read().iter().any(|f| f.flow == props.target) {
                    div { class: "empty-state",
                        div { class: "icon" }
                        div { class: "message", "データがありません" }
                        div { class: "sub-message", "新しい項目を追加してください" }
                    }
                } else {
                    ul { class: "items",
                        for flow in flows_sig.read().iter().filter(|f| f.flow == props.target) {
                            {
                                let fid = flow.id;
                                let flow_type = flow.flow.clone();
                                let flow_name = flow.name.clone();
                                let flow_created = flow.created_at.clone();
                                let flow_amount = flow.amount;
                                let is_active = selected_id().map(|id| id == fid).unwrap_or(false);
                                rsx! {
                                    li {
                                        class: if flow_type == "in" { if is_active { "income-item active" } else { "income-item" } } else { if is_active { "expense-item active" } else { "expense-item" } },
                                        onclick: move |_| {
                                            selected_id
                                                .set(
                                                    match selected_id() {
                                                        Some(id) if id == fid => None,
                                                        _ => Some(fid),
                                                    },
                                                );
                                        },
                                        div { class: "item-details",
                                            div { class: "name", "{flow_name}" }
                                            div { class: "date", "{flow_created}" }
                                        }
                                        div { class: "amount", "{flow_amount}" }
                                        if is_active {
                                            div { class: "actions flex gap-2 mt-2",
                                                button {
                                                    class: "px-2 py-1 text-xs rounded bg-red-600 hover:bg-red-700 text-white",
                                                    onclick: {
                                                        let flows = flows_from_props.clone();
                                                        move |_| {
                                                            let mut dac: DataAccess = DataAccess::new(fid, flows.clone());
                                                            spawn(async move {
                                                                dac.delete_specific_data().await;
                                                                dac.refresh().await;
                                                            });
                                                        }
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

#[derive(Clone)]
struct DataAccess {
    id: i32,
    flows: Vec<CashFlow>,
}

impl DataAccess {
    pub fn new(id: i32, flows: Vec<CashFlow>) -> Self {
        Self { id, flows }
    }

    async fn delete_specific_data(&self) {
        invoke::<bool>(
            "delete_specific_data",
            &serde_json::json!({ "id": self.id }),
        )
        .await;
    }

    async fn refresh(&mut self) {
        let latest: Vec<CashFlow> = invoke::<Vec<CashFlow>>(
            "list_cash_flows",
            &serde_json::json!({ "flows": self.flows }),
        )
        .await;
        self.flows = latest;
    }
}
