use dioxus::prelude::*;

const CSS_PATH:Asset = asset!("/assets/components/sidebar/sidebar.css");

#[derive(Debug, PartialEq, Clone, Props)]
struct NavItemInfo {
    pub name: &'static str,
    pub icon: Option<&'static str>,
    pub handle_click: EventHandler<MouseEvent>
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
    rsx!{
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "sidebar-container",
            div { class: "button-container",
                NavItem { name: "home", handle_click: move |_| test() }
                NavItem { name: "list", handle_click: move |_| test() }
            }
        }
    }
}

fn test() {

}
