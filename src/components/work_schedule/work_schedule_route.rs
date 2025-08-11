use super::setting_default_value::SettingDefaultValue;
use super::work_schedule::WorkSchedule;
use dioxus::prelude::*;
use serde_json;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn WorkScheduleRoute() -> Element {
    // オーバレイ（初期値設定）表示フラグ
    let mut show_settings: Signal<bool> = use_signal(|| false);
    use_future(move || async move {
        handle_load().await;
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "flex flex-col gap-4 w-[80vw] pt-4",
            // ヘッダー操作
            div { class: "flex flex-row gap-4 items-center mt-2",
                button {
                    class: "px-3 py-1 rounded bg-blue-600 hover:bg-blue-700 text-white w-[15vw]",
                    onclick: move |_| show_settings.set(true),
                    "初期値設定"
                }
            }

            // メイン（常に表示）
            WorkSchedule { on_submit: handle_submit }

            // オーバレイ（初期値設定）
            if show_settings() {
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
    }
}

async fn handle_load() {
    invoke::<bool>("init_work_schedule_db", &()).await;
}

async fn handle_submit(props: WorkRecord) {
    invoke::<bool>("add_work_schedule", &serde_json::json!({"props": props})).await;
}

async fn handle_submit_setting(props: WorkRecord) {
    invoke::<bool>("add_defaule_work_schedule", &serde_json::json!({"props": props})).await;
}
