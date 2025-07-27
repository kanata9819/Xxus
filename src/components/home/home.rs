use dioxus::prelude::*;

const CSS_PATH: Asset = asset!("/assets/components/home/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "home-container",
            h1 { "Welcome to the Home Page!" }
            p { "This is the main content area." }
        }
    }
}
