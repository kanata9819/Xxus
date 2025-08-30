use crate::enums::enum_global::AppRoute;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use wasm_bindgen::prelude::*;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> Element {
    rsx!(
        link { rel: "stylesheet", href: CSS_PATH }
        Router::<AppRoute> {}
    )
}
