use chrono::{NaiveDate};
use dioxus::prelude::*;
use shared_types::WorkRecord;
// use web_sys::console::log_1;
use payroll_core::calc_total_salary;

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
