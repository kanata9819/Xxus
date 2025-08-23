use super::timesheet_month_actuals::TimesheetMonthActuals;
use dioxus::prelude::*;

static CSS_PATH: Asset = asset!("assets/styles.css");

#[component]
pub fn WorkScheduleRoute() -> Element {

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "flex flex-col gap-4 w-[80vw] pt-4", TimesheetMonthActuals {} }
    }
}
