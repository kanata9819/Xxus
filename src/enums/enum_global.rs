use dioxus_router::prelude::*;
use dioxus::prelude::*;
use crate::components::home::home::Home;
use crate::components::settings::settings::Settings;
use crate::components::layout::main_layout::MainLayout;

#[derive(Debug, Routable, Clone, Copy, PartialEq, Eq)]
pub enum AppRoute {
    #[layout(MainLayout)]
    #[route("/")]
    Home,
    #[route("/settings")]
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowType {
    Income,
    Expense,
}
