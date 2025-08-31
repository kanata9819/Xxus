use dioxus::prelude::*;
use futures::StreamExt;
use tauri_sys::event::listen;
// use web_sys::console::log_1;

#[derive(Props, Clone, PartialEq)]
pub struct ImportCsvProps {
    pub visible: bool,
    pub on_close: EventHandler<()>,
}

#[derive(Clone, Copy)]
struct DragEventHandler {
    hovering: Signal<bool>,
}

impl DragEventHandler {
    fn new() -> Self {
        Self {
            hovering: use_signal(|| false),
        }
    }
    // ドラッグが要素上にあるとき
    fn handle_drag_over(mut self, event: Event<DragData>) {
        event.prevent_default();
        event.stop_propagation();
        self.hovering.set(true);
    }
    // ドラッグが要素外に出たとき
    fn handle_drag_leave(mut self, event: Event<DragData>) {
        event.prevent_default();
        event.stop_propagation();
        self.hovering.set(false);
    }
    // ドラッグ＆ドロップが完了したとき
    fn handle_drop(mut self, event: Event<DragData>) {
        event.prevent_default();
        event.stop_propagation();
        self.hovering.set(false);
        // ここでファイル処理を実装予定
    }
}

#[component]
pub fn ImportCsv(props: ImportCsvProps) -> Element {
    if !props.visible {
        return rsx!();
    }

    let drag_handler: DragEventHandler = DragEventHandler::new();
    let dropped_files: Signal<Vec<shared_types::DroppedFile>> = use_signal(|| vec![]);

    use_effect(move || {
        let mut df_cloned: Signal<Vec<shared_types::DroppedFile>> = dropped_files.clone();
        spawn(async move {
            if let Ok(mut stream) = listen::<Vec<shared_types::DroppedFile>>("file_dropped").await {
                while let Some(fileDropEvent) = stream.next().await {
                    df_cloned.set(fileDropEvent.payload);
                }
            }
        });
    });

    rsx! {
        div { class: "fixed inset-0 z-[10] bg-black/50 backdrop-blur-sm
        flex items-center justify-center
        animate-in fade-in duration-150 text-white",
            // ファイル情報表示エリア
            div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            flex flex-col items-center gap-4",
                div { class: "text-2xl font-bold", "CSVインポート" }
                for file in dropped_files.iter() {
                    "{file.name}"
                }
            }
            button {
                class: "btn-secondary mt-4",
                onclick: move |_| props.on_close.call(()),
                "閉じる"
            }
            // CSVドロップエリア
            div {
                class: "rounded-xl border-2 border-dashed p-12 w-[60vw] h-[30vh] transition-colors",
                ondragover: move |e| drag_handler.handle_drag_over(e),
                ondragleave: move |e| drag_handler.handle_drag_leave(e),
                ondrop: move |e| drag_handler.handle_drop(e),

                div { class: "text-5xl mb-2", "📄" }
                div { class: "font-medium", "ここにファイルをドロップ" }
                div { class: "text-xs text-slate-500 mt-1", ".csv だけを推奨" }
            }
        }
    }
}
