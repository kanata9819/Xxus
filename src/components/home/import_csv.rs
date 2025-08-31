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
        // オーバーレイ用コンテナ
        div { class: "fixed flex-col inset-0 z-[10] bg-black/50 backdrop-blur-sm flex items-center justify-center animate-in fade-in duration-150 text-white",

            // 要素整形用コンテナ
            div { class: "relative w-[80vw] h-[90vh] flex flex-col justify-center items-center gap-6 bg-slate-800/80 rounded-xl p-8",

                button {
                    class: "btn-primary mt-4 right-0",
                    onclick: move |_| props.on_close.call(()),
                    "閉じる"
                }

                // ファイル情報表示エリア
                div { class: "top-1/2 left-1/2 flex flex-col items-center gap-4 border-2 w-[60vw] h-[30vh] border-slate-600 pb-4 rounded-lg",

                    for file in dropped_files.iter() {

                        div { class: "flex justify-between items-center flex-row p-4 w-[50vw] border-2 border-slate-600 rounded-lg",
                            "{file.name}"
                            button { class: "btn-primary right-0", "取り込み" }
                        }
                    }
                }

                // CSVドロップエリア
                div {
                    class: "rounded-xl border-2 border-dashed p-12 w-[60vw] h-[30vh] transition-colors",
                    ondragover: move |e| drag_handler.handle_drag_over(e),
                    ondragleave: move |e| drag_handler.handle_drag_leave(e),
                    ondrop: move |e| drag_handler.handle_drop(e),

                    div { class: "flex flex-col items-center justify-center h-full",
                        div { class: "text-5xl mb-2", "📄" }
                        div { class: "font-medium", "ここにファイルをドロップ" }
                        div { class: "text-xs text-slate-500 mt-1", ".csv だけを推奨" }
                    }
                }
            }
        }
    }
}
