use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use shared_types::WorkRecord;
use web_sys::console::log_1;

#[component]
pub fn SalaryPreview(
    work_data: Signal<Vec<WorkRecord>>,
    selected_date: Signal<NaiveDate>,
) -> Element {
    let mut total_salary: Signal<i32> = use_signal(|| 0);

    use_effect(move || {
        log_1(&format!("selected_date: {}", selected_date.read()).into());
        total_salary.set(calc_total_salary(&work_data.read(), &selected_date.read()));
    });

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
    log_1(&format!("Calculating total salary for: {}", display_ym).into());

    for record in work_data.iter() {
        let record_ymd: NaiveDate = record.date.parse::<NaiveDate>().unwrap_or_else(|_| {
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
        let record_ym: String = format!("{}{:02}", record_ymd.year(), record_ymd.month());
        log_1(&format!("Recordym {}", record_ym).into());

        if record_ym == display_ym {
            total += record.amount;
        }
    }

    total
}
