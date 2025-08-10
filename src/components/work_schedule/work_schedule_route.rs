use super::work_schedule::WorkSchedule;
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

#[component]
pub fn WorkScheduleRoute() -> Element {
    use_future(move || async move {
        handle_load().await;
    });

    rsx! {
        WorkSchedule { on_submit: handle_submit }
    }
}

async fn handle_load() {
    invoke::<bool>("init_work_schedule_db", &()).await;
}

async fn handle_submit(props: WorkRecord) {
    invoke::<bool>("add_work_schedule", props).await;
}
