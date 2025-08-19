use dioxus::prelude::*;
use shared_types::CashFlow;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("/assets/components/home/list.css");

#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    pub flows: Vec<CashFlow>,
    pub target: String,
}

#[component]
pub fn List(props: ListProps) -> Element {
    // 選択中アイテム (id) を保持。None なら未選択
    let mut selected_id: Signal<Option<i32>> = use_signal(|| None);

    // flows をローカルにクローン (イベントクロージャ 'static 制約を満たすため)
    let mut flows_sig: Signal<Vec<CashFlow>> = use_signal(|| Vec::new());

    use_effect (move || {
        if props.flows.is_empty() {
            return;
        }
        flows_sig.set(props.flows.clone());
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "list-container",
            div { class: "item-info",
                if !(flows_sig)().iter().any(|f| f.flow == props.target) {
                    div { class: "empty-state",
                        div { class: "icon" }
                        div { class: "message", "データがありません" }
                        div { class: "sub-message", "新しい項目を追加してください" }
                    }
                } else {
                    ul { class: "items",
                        for flow in flows_sig().iter().filter(|f| f.flow == props.target) {
                            {
                                let fid = flow.id;
                                let flow_type = flow.flow.clone();
                                let flow_name = flow.name.clone();
                                let flow_created = flow.created_at.clone();
                                let flow_amount = flow.amount;
                                let is_active = selected_id().map(|id| id == fid).unwrap_or(false);
                                let dac: DataAccess = DataAccess::new(fid, flows_sig.clone());
                                rsx! {
                                    li {
                                        class: if flow_type == "in" { if is_active { "income-item active" } else { "income-item" } } else { if is_active { "expense-item active" } else { "expense-item" } },
                                        onclick: move |_| {
                                            selected_id
                                                .set(
                                                    if selected_id().is_some() && selected_id().unwrap() == fid {
                                                        None
                                                    } else {
                                                        Some(fid)
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
                                                    onclick: move |_| {
                                                        let dac: DataAccess = dac.clone();
                                                        async move {
                                                            dac.delete_specific_data().await;
                                                            dac.refresh().await;
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
    flows: Signal<Vec<CashFlow>>,
}

impl DataAccess {
    pub fn new(id: i32, flows: Signal<Vec<CashFlow>>) -> Self {
        Self { id, flows }
    }

    async fn delete_specific_data(&self) {
        invoke::<bool>(
            "delete_specific_data",
            &serde_json::json!({ "id": self.id }),
        )
        .await;
    }

    async fn refresh(&self) {
        let latest: Vec<CashFlow> = invoke::<Vec<CashFlow>>(
            "list_cash_flows",
            &serde_json::json!({ "flows": (self.flows)() }),
        )
        .await;
        let mut flows: Signal<Vec<CashFlow>> = self.flows.clone();
        flows.set(latest);
    }
}
