use dioxus::prelude::*;
// use web_sys::console::log_1;

#[derive(Props, Clone, PartialEq)]
pub struct ImportCsvProps {
    pub visible: bool,
    pub on_close: EventHandler<MouseEvent>,
}

#[component]
pub fn ImportCsv(props: ImportCsvProps) -> Element {

    if !props.visible {
        return rsx!()
    }

    rsx! {
        div { class: "fixed inset-0 z-[10] bg-black/50 backdrop-blur-sm
                    flex items-center justify-center
                    animate-in fade-in duration-150 text-white",
            "CSVインポート"
            button {
                class: "btn-secondary mt-4",
                onclick: move |e: MouseEvent| props.on_close.call(e),
                "閉じる"
            }
        }
    }
}
