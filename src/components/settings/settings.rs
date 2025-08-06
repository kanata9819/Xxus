use dioxus::prelude::*;
use tauri_sys::core::invoke;
use serde_json::json;

const CSS_PATH: Asset = asset!("/assets/components/settings/settings.css");

#[component]
pub fn Settings() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "settings-container",
            button {
                class: "button",
                onclick: async move |_: Event<MouseData>| { delete_whole_data().await },
                id: "init-db",
                "initialize database"
            }
            h1 { "Settings" }
            p { "Adjust your preferences here." }
        }
    }
}

async fn delete_whole_data() {
    invoke::<()>("delete_whole_data", &json!({})).await;
}
