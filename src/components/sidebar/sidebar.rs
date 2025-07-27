use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::enums::enum_global::AppRoute;
use dioxus_material_icons::{MaterialIconStylesheet, MaterialIcon};

const CSS_PATH: Asset = asset!("/assets/components/sidebar/sidebar.css");

#[derive(Debug, PartialEq, Clone, Props)]
struct NavItemInfo {
    pub name: &'static str,
    pub icon: Option<&'static str>,
    pub handle_click: EventHandler<MouseEvent>,
}

#[component]
fn NavItem(props: NavItemInfo) -> Element {
    rsx!(
        button {
            id: props.name,
            class: "sidebar-button",
            onclick: props.handle_click,
            icon: props.icon,
            MaterialIcon { name: props.name, size: Some(24) }
        }
    )
}

#[component]
pub fn Sidebar() -> Element {
    let nav: Navigator = use_navigator();

    rsx! {
        MaterialIconStylesheet {}
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "sidebar-container",
            div { class: "button-container",
                NavItem {
                    name: "home",
                    handle_click: move |_| navigate(&nav, AppRoute::Home),
                }
                NavItem {
                    name: "settings",
                    handle_click: move |_| navigate(&nav, AppRoute::Settings),
                }
            }
        }
    }
}

fn navigate(nav: &Navigator, target: AppRoute) {
    nav.push(target);
}
