use dioxus::prelude::*;
use chrono::prelude::*;

#[component]
pub fn TimesheetMonthActuals() -> Element {

    let calender = {
        let date = Local::now();
    };

    rsx! {
        div { "Timesheet Month Actuals" }
    }
}
