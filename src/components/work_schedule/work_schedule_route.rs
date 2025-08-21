use super::overlay::Overlay;
use super::work_schedule::WorkSchedule;
use super::timesheet_month_actuals::TimesheetMonthActuals;
use dioxus::prelude::*;
use serde_json;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn WorkScheduleRoute() -> Element {
    // オーバレイ（初期値設定）表示フラグ
    let mut show_settings: Signal<bool> = use_signal(|| false);

    let mut show_list: Signal<bool> = use_signal(|| false);
    // トースト（メッセージ, is_error）
    let mut toast: Signal<Option<(String, bool)>> = use_signal(|| None);
    use_future(move || async move {
        handle_load().await;
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "flex flex-col gap-4 w-[80vw] pt-4",
            // ヘッダー操作
            div { class: "flex flex-row gap-4 items-center mt-2",
                button {
                    class: "px-3 py-1 rounded bg-blue-600 hover:bg-blue-700 text-white w-[12vw] h-[7vh]",
                    onclick: move |_| show_settings.set(true),
                    "初期値設定"
                }
                match show_list() {
                    true => rsx! {
                        button {
                            class: "px-3 py-1 rounded bg-blue-600 hover:bg-blue-700 text-white w-[12vw] h-[7vh]",
                            onclick: move |_| show_list.set(false),
                            "入力画面表示"
                        }
                    },
                    false => rsx! {
                        button {
                            class: "px-3 py-1 rounded bg-blue-600 hover:bg-blue-700 text-white w-[12vw] h-[7vh]",
                            onclick: move |_| show_list.set(true),
                            "一覧表示"
                        }
                    },
                }
            }

            match show_list() {
                true => rsx! {
                    TimesheetMonthActuals {}
                },
                false => rsx! {
                    // メイン（常に表示）
                    WorkSchedule {
                        on_submit: move |props: WorkRecord| {
                            let mut toast_set = toast.clone();
                            spawn(async move {
                                let ok: bool = invoke::<
                                    bool,
                                >("add_work_schedule", &serde_json::json!({ "props" : props }))
                                    .await;
                                if ok {
                                    toast_set.set(Some(("登録に成功しました".to_string(), false)));
                                } else {
                                    toast_set.set(Some(("登録に失敗しました".to_string(), true)));
                                }
                            });
                        },
                    }
                    
                    // トースト表示
                    match toast() {
                        Some((ref msg, _is_err)) => rsx! {
                            div { class: "fixed bottom-4 right-4 z-50",
                                div { class: "modal-panel-dark border rounded shadow px-4 py-2 flex items-center gap-3",
                                    span { class: "text-sm", "{msg}" }
                                    button {
                                        class: "text-gray-500 hover:text-gray-300",
                                        onclick: move |_| toast.set(None),
                                        "×"
                                    }
                                }
                            }
                        },
                        None => rsx! {},
                    }
                    
                    // オーバレイ（初期値設定）
                    if show_settings() {
                        Overlay {
                            show_settings: show_settings.clone(),
                            on_toast: move |(msg, is_err): (String, bool)| {
                                let mut t = toast.clone();
                                t.set(Some((msg, is_err)));
                            },
                        }
                    }
                },
            }
        }
    }
}

async fn handle_load() {
    invoke::<bool>("init_work_schedule_db", &()).await;
    invoke::<bool>("init_default_value_db", &()).await;
}
