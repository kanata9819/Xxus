use dioxus::prelude::*;
use shared_types::CashFlow;

static CSS_PATH: Asset = asset!("assets/components/home/list.css");

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
                                let li_class: &str = if flow_type == "in" {
                                    if is_active {
                                        "income-item active group relative flex items-start gap-4 rounded-xl px-5 py-4 border border-emerald-400/50 bg-emerald-500/10 hover:bg-emerald-500/20 ring-1 ring-emerald-400 shadow-sm transition-colors duration-200"
                                    } else {
                                        "income-item group relative flex items-start gap-4 rounded-xl px-5 py-4 border border-white/15 bg-white/5 hover:bg-emerald-500/10 hover:border-emerald-400/40 shadow-sm transition-colors duration-200"
                                    }
                                } else {
                                    if is_active {
                                        "expense-item active group relative flex items-start gap-4 rounded-xl px-5 py-4 border border-rose-400/50 bg-rose-500/10 hover:bg-rose-500/20 ring-1 ring-rose-400 shadow-sm transition-colors duration-200"
                                    } else {
                                        "expense-item group relative flex items-start gap-4 rounded-xl px-5 py-4 border border-white/15 bg-white/5 hover:bg-rose-500/10 hover:border-rose-400/40 shadow-sm transition-colors duration-200"
                                    }
                                };
                                let amount_class: &str = if flow_type == "in" {
                                    "amount text-emerald-400 font-semibold tracking-wide text-sm md:text-base"
                                } else {
                                    "amount text-rose-400 font-semibold tracking-wide text-sm md:text-base"
                                };
                                rsx! {
                                    li {
                                        class: "{li_class}",
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
                                            div { class: "name font-medium text-slate-100 text-sm md:text-base leading-tight",
                                                "{flow_name}"
                                            }
                                            div { class: "date text-[11px] md:text-xs text-slate-400 tracking-wide", "{flow_created}" }
                                        }
                                        div { class: "{amount_class}", "{flow_amount}" }
                                        if is_active {
                                            div { class: "flex gap-2 ml-2 items-center animate-in fade-in duration-200",
                                                button {
                                                    class: "px-2 py-1 text-[10px] md:text-xs rounded-md font-medium bg-red-500/90 hover:bg-red-500 text-white shadow-sm hover:shadow ring-1 ring-red-400/40 transition active:scale-[0.97]",
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
