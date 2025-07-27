use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::enums::enum_global::AppRoute;
use dioxus_router::prelude::Router;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> Element {
    rsx!(
        Router::<AppRoute> {}
    )
}
