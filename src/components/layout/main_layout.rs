use crate::components::sidebar::sidebar_panel::Sidebar;
use crate::enums::enum_global::AppRoute;
use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;

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
