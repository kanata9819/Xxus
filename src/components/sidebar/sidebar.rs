use crate::components::home::home::Home;
use crate::components::settings::settings::Settings;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

const CSS_PATH: Asset = asset!("/assets/components/sidebar/sidebar.css");

#[derive(Debug, Routable, Clone, Copy, PartialEq, Eq)]
enum AppRoute {
    #[route("/")]
    Home,
    #[route("/settings")]
    Settings,
}

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
            class: "item-name",
            onclick: props.handle_click,
            icon: props.icon,
            {props.name}
        }
    )
}

#[component]
pub fn Sidebar() -> Element {
    let nav: Navigator = use_navigator();

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "sidebar-container",
            div { class: "button-container",
                NavItem {
                    name: "home",
                    handle_click: move |_| navigate(&nav, AppRoute::Home),
                }
                NavItem {
                    name: "Settings",
                    handle_click: move |_| navigate(&nav, AppRoute::Settings),
                }
            }
        }
    }
}

fn navigate(nav: &Navigator, target: AppRoute) {
    nav.push(target);
}
