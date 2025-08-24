use super::overlay::Overlay;
use super::work_schedule::WorkSchedule;
use chrono::{prelude::*, Duration};
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;
use web_sys::console::log_1;

#[component]
pub fn TimesheetMonthActuals() -> Element {
    let today: NaiveDate = Local::now().date_naive();
    let current_year: i32 = today.year();
    let current_month: u32 = today.month();
    let current_day: u32 = today.day();

    let mut work_data: Signal<Vec<WorkRecord>> = use_signal(|| vec![]);
    let mut toast: Signal<Option<(String, bool)>> = use_signal(|| None);
    let show_settings: Signal<bool> = use_signal(|| false);
    // 当月初日の年月日
    let base_YMD: Option<NaiveDate> = NaiveDate::from_ymd_opt(current_year, current_month, 1);
    // 来月初日の年月日
    let base_start_next_month_YMD: Option<NaiveDate> =
        NaiveDate::from_ymd_opt(current_year, current_month + 1, 1);
    // 当月末日の年月日
    let base_end_YMD: NaiveDate = base_start_next_month_YMD.unwrap() - Duration::days(1);
    // 実績入力画面を表示するか
    let mut show_input: Signal<bool> = use_signal(|| false);

    use_future(move || async move {
        let init_result: bool =
            invoke::<bool>("init_work_schedule_db", &serde_json::json!({})).await;

        if init_result {
            let fetched_data: Vec<WorkRecord> =
                invoke::<Vec<WorkRecord>>("get_work_schedule_data", &serde_json::json!({})).await;

            if fetched_data.is_empty() {
                toast.set(Some((
                    "実績データがありません。勤務入力から登録してください".to_string(),
                    true,
                )));
            } else {
                work_data.set(fetched_data);
            }
        }
    });

    rsx! {
        // ヘッダー部
        div { class: "mb-4 flex items-end gap-3",
            div { class: "text-2xl font-semibold tracking-wide text-slate-100",
                "{current_year}"
                span { class: "ml-1 text-base font-normal text-slate-400", "年" }
            }
            div { class: "text-2xl font-semibold text-slate-100",
                "{current_month}"
                span { class: "ml-1 text-base font-normal text-slate-400", "月" }
            }
            div { class: "ml-auto flex items-center gap-3",
                div { class: "px-3 py-1 text-xs rounded-full bg-slate-700/50 text-slate-300 ring-1 ring-white/10",
                    "本日: {current_day}日"
                }
                button {
                    class: "px-3 py-1.5 rounded-md bg-sky-600/80 hover:bg-sky-600 text-white text-xs font-semibold tracking-wide shadow transition",
                    onclick: move |_| show_input.set(true),
                    "勤務入力"
                }
            }
        }

        // 日付グリッド
        div { class: "grid gap-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5",
            for day in 1..=base_end_YMD.day() {
                {
                    let disp_date: NaiveDate = NaiveDate::from_ymd_opt(
                            base_YMD.map_or(0, |d| d.year()),
                            base_YMD.map_or(0, |d| d.month()),
                            day,
                        )
                        .unwrap();
                    let weekday = match disp_date.weekday() {
                        Weekday::Mon => "月",
                        Weekday::Tue => "火",
                        Weekday::Wed => "水",
                        Weekday::Thu => "木",
                        Weekday::Fri => "金",
                        Weekday::Sat => "土",
                        Weekday::Sun => "日",
                    };
                    let is_today = disp_date.day() == current_day;
                    let base_color = match weekday {
                        "土" => "text-blue-400",
                        "日" => "text-rose-400",
                        _ => "text-slate-200",
                    };
                    let bg_color = match weekday {
                        "土" => "bg-blue-500/5 hover:bg-blue-500/10",
                        "日" => "bg-rose-500/5 hover:bg-rose-500/10",
                        _ => "bg-white/5 hover:bg-white/10",
                    };
                    let ring_color = if is_today { "ring-emerald-400/60" } else { "ring-white/10" };
                    let today_badge = if is_today { Some("TODAY") } else { None };
                    rsx! {
                        div {
                            class: "group relative rounded-lg p-4 flex flex-col gap-1 transition-colors {bg_color} ring-1 {ring_color} shadow-sm",
                            onclick: move |_| {
                                show_input.set(true);
                            },
                            div { class: "flex items-baseline gap-2",
                                span { class: "text-sm font-semibold tracking-wide {base_color}", "{current_month}月{day}日" }
                                span { class: "text-[10px] md:text-[11px] tracking-wider text-slate-400 group-hover:text-slate-300 transition",
                                    "({weekday})"
                                }
                                if let Some(b) = today_badge {
                                    span { class: "ml-auto px-1.5 py-0.5 text-[10px] rounded-md bg-emerald-500/20 text-emerald-300 ring-1 ring-emerald-400/30",
                                        "{b}"
                                    }
                                }
                            }
                            // 将来: 実績データセクション
                            div { class: "h-5 text-[11px] group-hover:text-slate-400 italic",
                                if check_data_exists(&disp_date, &work_data.read()) {
                                    div { class: "text-green-500", "記録あり" }
                                } else {
                                    div { class: "text-slate-500", "記録なし" }
                                }
                            }
                        }
                    }
                }
            }
        }
        // 勤務入力オーバーレイ
        if *show_input.read() {
            div { class: "fixed inset-0 z-40",
                // 背景
                div {
                    class: "absolute inset-0 bg-black/50 backdrop-blur-sm",
                    onclick: move |_| show_input.set(false),
                }
                // パネル
                div { class: "absolute inset-0 flex items-center justify-center p-4",
                    div { class: "w-[100vw] max-w-[1250px] max-h-[90vh] overflow-y-auto rounded-xl bg-slate-900/95 border border-white/10 ring-1 ring-white/15 shadow-2xl backdrop-blur-xl flex flex-col",
                        div { class: "p-5 pb-6",
                            WorkSchedule {
                                on_submit: move |props: WorkRecord| {
                                    let mut toast_set = toast.clone();
                                    let mut close_flag = show_input.clone();
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
                                        close_flag.set(false);
                                    });
                                },
                                show_input: show_input.clone(),
                                show_settings: show_settings.clone(),
                                timesheet_data: work_data.read().to_vec(),
                            }
                        }
                    }
                }
            }
        }

        // 設定オーバーレイ
        if *show_settings.read() {
            Overlay {
                show_settings: show_settings.clone(),
                on_toast: move |(msg, is_err): (String, bool)| {
                    toast.set(Some((msg, is_err)));
                },
            }
        }

        // トースト表示 (常時)
        match *toast.read() {
            Some((ref msg, is_show)) => rsx! {
                if is_show {
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
                }
            },
            None => rsx! {},
        }
    }
}

fn check_data_exists(disp_date: &NaiveDate, work_data: &Vec<WorkRecord>) -> bool {
    log_1(&format!("Checking data for date: {}", disp_date).into());
    log_1(&format!("Total records: {}", work_data.len()).into());
    for record in work_data {
        log_1(&format!("Checking record date: {}", record.date).into());
        if record.date == disp_date.to_string() {
            return true;
        }
    }
    false
}
