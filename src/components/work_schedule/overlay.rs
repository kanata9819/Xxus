use super::setting_default_value::SettingDefaultValue;
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn Overlay(show_settings: Signal<bool>) -> Element {
    let mut show_settings = show_settings.clone();

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        Fragment {
            // 背景（クリックで閉じる）
            div {
                class: "fixed inset-0 bg-black/40 backdrop-blur-[1px] z-40",
                onclick: move |_| show_settings.set(false),
            }
            // モーダル本体
            div { class: "fixed inset-0 z-50 flex items-center justify-center p-[4]",
                div { class: "pointer-events-auto modal-panel-dark rounded-lg shadow-xl w-[90vw] max-w-[800px] max-h-[85vh] overflow-hidden border border-gray-200",
                    // タイトルバー
                    div { class: "flex items-center justify-between px-4 py-2 border-b",
                        h3 { class: "font-semibold", "初期値設定" }
                        button {
                            class: "text-gray-500 hover:text-gray-700 text-xl leading-none",
                            onclick: move |_| show_settings.set(false),
                            "×"
                        }
                    }
                    // コンテンツ（スクロール可）
                    div { class: "p-4 overflow-y-auto max-h-[75vh]",
                        SettingDefaultValue { on_submit: handle_submit_setting }
                    }
                }
            }
        }
    }
}

async fn handle_submit_setting(props: WorkRecord) {
    invoke::<bool>(
        "add_defaule_work_schedule",
        &serde_json::json!({"props": props}),
    )
    .await;
}
