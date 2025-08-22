use chrono::{prelude::*, Duration};
use dioxus::{prelude::*};
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
        div { class: "flex flex-row gap-2",
            div { "{current_year}年" }
            div { "{current_month}月" }
        }

        for day in 1..=base_end_YMD.day() {
            {
                let disp_date = NaiveDate::from_ymd_opt(
                    base_YMD.map_or(0, |d| d.year()),
                    base_YMD.map_or(0, |d| d.month()),
                    day,
                );
                let disp_date_weekday = match disp_date.unwrap().weekday() {
                    Weekday::Mon => "月",
                    Weekday::Tue => "火",
                    Weekday::Wed => "水",
                    Weekday::Thu => "木",
                    Weekday::Fri => "金",
                    Weekday::Sat => "土",
                    Weekday::Sun => "日",
                };
                let date_color = match disp_date_weekday {
                    "土" => "text-blue-600",
                    "日" => "text-red-600",
                    _ => "text-white",
                };
                rsx! {
                    div { class: "flex flex-row {date_color}",
                        "{base_end_YMD.month()}月{day}日"
                        div { "({disp_date_weekday})" }
                    }
                }
            }
        }
    }
}
