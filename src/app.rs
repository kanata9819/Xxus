use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::sidebar::sidebar::Sidebar;
use crate::components::home::home::Home;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

const CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn App() -> Element {
    rsx!(
        Sidebar {}
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "main-container", Home {} }
    )
}
