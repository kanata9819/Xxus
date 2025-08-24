use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::enums::enum_global::AppRoute;
// use dioxus_material_icons::{MaterialIconStylesheet, MaterialIcon}; // コメントアウト: リリースで不具合切り分けのため無効化

const CSS_PATH: Asset = asset!("/assets/components/sidebar/sidebar.css");

#[derive(Debug, PartialEq, Clone, Props)]
struct NavItemInfo {
    pub name: &'static str,
    pub icon: Option<&'static str>,
    pub handle_click: EventHandler<MouseEvent>,
    pub active: bool,
}

#[component]
fn NavItem(props: NavItemInfo) -> Element {
    rsx!(
        button {
            id: props.name,
            class: if props.active { "sidebar-button active" } else { "sidebar-button" },
            onclick: props.handle_click,
            // icon: props.icon,
            span { class: "inline-block w-6 text-center mr-1", {
                match props.name {
                    "home" => "🏠",
                    "work" => "🗂",
                    "settings" => "⚙️",
                    _ => "•",
                }
            }}
            span { class: "text-xs tracking-wide", { props.name.to_uppercase() } }
        }
    )
}

#[component]
pub fn Sidebar() -> Element {
    let nav: Navigator = use_navigator();
    let current: AppRoute = use_route::<AppRoute>();

    rsx! {
    // MaterialIconStylesheet {} // コメントアウト: フォント/アイコン依存を一時的に外す
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "sidebar-container",
            div { class: "button-container",
                NavItem {
                    name: "home",
                    active: current == AppRoute::Home,
                    handle_click: move |_| navigate(&nav, AppRoute::Home),
                }
                NavItem {
                    name: "work",
                    active: current == AppRoute::WorkScheduleRoute,
                    handle_click: move |_| navigate(&nav, AppRoute::WorkScheduleRoute),
                }
                NavItem {
                    name: "settings",
                    active: current == AppRoute::Settings,
                    handle_click: move |_| navigate(&nav, AppRoute::Settings),
                }
            }
        }
    }
}

fn navigate(nav: &Navigator, target: AppRoute) {
    nav.push(target);
}
