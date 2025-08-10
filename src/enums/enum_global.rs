use dioxus_router::prelude::*;
use dioxus::prelude::*;
use crate::components::home::home::Home;
use crate::components::settings::settings::Settings;
use crate::components::layout::main_layout::MainLayout;
use crate::components::work_schedule::work_schedule_route::{WorkScheduleRoute};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowType {
    Income,
    Expense,
}
