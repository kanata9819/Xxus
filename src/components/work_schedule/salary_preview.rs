use chrono::NaiveDate;
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

#[component]
pub fn SalaryPreview(
    work_data: Signal<Vec<WorkRecord>>,
    display_month: Signal<NaiveDate>,
) -> Element {
    let total_salary: Signal<i32> = use_signal(|| 0);

    use_effect(move || {
        let wd: Vec<WorkRecord> = work_data.read().clone(); // VecはClone必須
        let sd: NaiveDate = *display_month.read(); // NaiveDateはCopy

        let mut total_salary_sig = total_salary.to_owned();

        spawn(async move {
            let calc_result: i32 = invoke::<i32>(
                "calc_total_salary",
                // Tauri の引数名マッピング: Rust 側 work_data -> JS 側 workData / selected_date -> selectedDate
                &serde_json::json!({ "workData": wd, "selectedDate": sd }),
            )
            .await;

            total_salary_sig.set(calc_result);
        });
    });

    rsx! {
        div { class: "p-4 border rounded mb-4 max-w-md",
            div { class: "text-lg font-bold mb-2", "合計給与" }
            div { class: "text-2xl font-semibold", "{total_salary}円" }
        }
    }
}
