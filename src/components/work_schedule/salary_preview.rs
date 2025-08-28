use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use shared_types::WorkRecord;
// use web_sys::console::log_1;

#[component]
pub fn SalaryPreview(
    work_data: Signal<Vec<WorkRecord>>,
    selected_date: Signal<NaiveDate>,
) -> Element {
    // 月変更やレコード変更ごとに常に最新を計算 (データ量が増えたら再度最適化検討)
    let total_salary = {
        let wd = work_data.read();
        let sd = selected_date.read();
        calc_total_salary(&wd, &sd)
    };

    rsx! {
        div { class: "p-4 border rounded mb-4 max-w-md",
            div { class: "text-lg font-bold mb-2", "合計給与" }
            div { class: "text-2xl font-semibold", "{total_salary}円" }
        }
    }
}

fn calc_total_salary(work_data: &[WorkRecord], display: &NaiveDate) -> i32 {
    let mut total: i32 = 0;
    let display_ym: String = format!("{}{:02}", display.year(), display.month());

    for record in work_data.iter() {
        let record_ymd: NaiveDate = record
            .date
            .parse::<NaiveDate>()
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let record_ym: String = format!("{}{:02}", record_ymd.year(), record_ymd.month());

        if record_ym == display_ym {
            total += record.amount;
        }
    }

    total
}
