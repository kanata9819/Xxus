use super::overlay::Overlay;
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
                    class: "px-3 py-1 rounded bg-blue-600 hover:bg-blue-700 text-white w-[10vw]",
                    onclick: move |_| show_settings.set(true),
                    "初期値設定"
                }
            }

            // メイン（常に表示）
            WorkSchedule { on_submit: handle_submit }

            // オーバレイ（初期値設定）
            if show_settings() {
                Overlay { show_settings: show_settings.clone() }
            }
        }
    }
}

async fn handle_load() {
    invoke::<bool>("init_work_schedule_db", &()).await;
    invoke::<bool>("init_default_value_db", &()).await;
}

async fn handle_submit(props: WorkRecord) {
    invoke::<bool>("add_work_schedule", &serde_json::json!({"props": props})).await;
}
