use chrono::{prelude::*, Duration};
use dioxus::prelude::*;
// use web_sys::console::log_1;

#[component]
pub fn TimesheetMonthActuals() -> Element {
    let today = Local::now().date_naive();
    let current_year = today.year();
    let current_month = today.month();
    let current_day = today.day();

    // 当月初日の年月日
    let base_YMD = NaiveDate::from_ymd_opt(current_year, current_month, 1);
    // 来月初日の年月日
    let base_start_next_month_YMD = NaiveDate::from_ymd_opt(current_year, current_month + 1, 1);
    // 当月末日の年月日
    let base_end_YMD = base_start_next_month_YMD.unwrap() - Duration::days(1);

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
            div { class: "ml-auto px-3 py-1 text-xs rounded-full bg-slate-700/50 text-slate-300 ring-1 ring-white/10",
                "本日: {current_day}日"
            }
        }

        // 日付グリッド
        div { class: "grid gap-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5",
            for day in 1..=base_end_YMD.day() {
                {
                    let disp_date = NaiveDate::from_ymd_opt(
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
                        div { class: "group relative rounded-lg p-3 flex flex-col gap-1 transition-colors {bg_color} ring-1 {ring_color} shadow-sm",
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
                            // 将来: 実績データセル
                            div { class: "h-5 text-[11px] text-slate-500 group-hover:text-slate-400 italic",
                                "記録なし"
                            }
                        }
                    }
                }
            }
        }
    }
}
