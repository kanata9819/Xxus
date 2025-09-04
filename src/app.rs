use crate::enums::enum_global::AppRoute;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
    rsx!(
        link { rel: "stylesheet", href: CSS_PATH }
        Router::<AppRoute> {}
    )
}
