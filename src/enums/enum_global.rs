use crate::components::home::home_panel::Home;
use crate::components::layout::main_layout::MainLayout;
use crate::components::settings::settings_panel::Settings;
use crate::components::work_schedule::work_schedule_route::WorkScheduleRoute;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Routable, Clone, Copy, PartialEq, Eq)]
pub enum AppRoute {
    #[layout(MainLayout)]
    #[route("/")]
    Home,
    #[route("/settings")]
    Settings,
    #[route("/work_schedule")]
    WorkScheduleRoute,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowType {
    Income,
    Expense,
}
