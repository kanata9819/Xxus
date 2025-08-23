use dioxus::prelude::*;
use dioxus_router::prelude::{Outlet};
use crate::components::sidebar::sidebar::Sidebar;
use crate::enums::enum_global::AppRoute;

const CSS_PATH: Asset = asset!("assets/styles.css");

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }

        div { class: "app-root",
            div { class: "sidebar", Sidebar {} }
            div { class: "main-container", style: "padding-top: 5vh;", Outlet::<AppRoute> {} }
        }
    }
}

