use super::work_schedule::{WorkSchedule};
use super::setting_default_value::SettingDefaultValue;
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;
use serde_json;

#[component]
pub fn WorkScheduleRoute() -> Element {
    let mut isOpenSetting: Signal<bool> = use_signal(|| false);
    use_future(move || async move {
        handle_load().await;
    });

    rsx! {
        button { id: "open_setting", onclick: move |_| isOpenSetting.set(true), "設定" }
        if isOpenSetting() {
            SettingDefaultValue { on_submit: handle_submit_setting }
        }
        WorkSchedule { on_submit: handle_submit }
    }
}

async fn handle_load() {
    invoke::<bool>("init_work_schedule_db", &()).await;
}

async fn handle_submit(props: WorkRecord) {
    invoke::<bool>("add_work_schedule", &serde_json::json!({"props": props})).await;
}

async fn handle_submit_setting(props: WorkRecord) {
    invoke::<bool>("add_work_schedule", &serde_json::json!({"props": props})).await;
}
