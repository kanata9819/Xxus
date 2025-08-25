use dioxus::{prelude::*};
use shared_types::WorkRecord;

#[component]
pub fn SalaryPreview(work_data: Vec<WorkRecord>) -> Element {

    let total_salary: i32 = work_data.iter().map(|record| record.amount).sum();

    rsx! {
        div { class: "p-4 border rounded mb-4 max-w-md",
            div { class: "text-lg font-bold mb-2", "合計給与" }
            div { class: "text-2xl font-semibold", "{total_salary}円" }
        }
    }
}
