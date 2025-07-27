use dioxus::prelude::*;

const CSS_PATH: Asset = asset!("/assets/components/settings/settings.css");

#[component]
pub fn Settings() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "settings-container",
            h1 { "Settings" }
            p { "Adjust your preferences here." }
        }
    }
}
