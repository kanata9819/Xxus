mod csv;
mod data_access;

use crate::data_access::db_path as resolved_db_path;
use chrono::NaiveDate;
use data_access::{
    data_access as dac, init_pool, setting_default_value as sdv, work_schedule as ws,
};
use payroll_core as payroll;
use shared_types::{AddCashFlowProps, CashFlow, WorkRecord};
use std::fs;
use tauri::WindowEvent;
use tauri::{DragDropEvent, Emitter, Manager};

#[tauri::command]
async fn init_db() -> bool {
    if init_pool().await.is_err() {
        return false;
    }
    dac::init_db().await.is_ok()
}

#[tauri::command]
fn db_path() -> Option<String> {
    resolved_db_path().map(|p| p.display().to_string())
}

//==============HOME==================================
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
async fn list_cash_flows() -> Result<Vec<CashFlow>, String> {
    dac::list_cash_flows().await
}

#[tauri::command(async)]
async fn add_cash_flow(props: AddCashFlowProps) -> Result<bool, String> {
    dac::add_cash_flow(props).await
}

#[tauri::command]
async fn delete_whole_data() -> Result<(), String> {
    dac::delete_whole_data().await
}

#[tauri::command]
async fn delete_specific_data(id: i32) -> Result<bool, String> {
    dac::delete_specific_data(id).await
}

//===============WORKSCHEDULE=================================
#[tauri::command]
async fn init_work_schedule_db() -> bool {
    if init_pool().await.is_err() {
        return false;
    }
    ws::init_db().await.is_ok()
}

#[tauri::command]
async fn add_work_schedule(props: WorkRecord) -> Result<bool, String> {
    ws::add_work_schedule(props).await
}

#[tauri::command]
async fn update_work_schedule(date: String, props: WorkRecord) -> Result<bool, String> {
    ws::update_work_schedule(date, props).await
}

#[tauri::command]
async fn delete_work_schedule_data() -> Result<(), String> {
    ws::delete_work_schedule_data().await
}

#[tauri::command]
async fn delete_specific_schedule_data(date: String) -> Result<bool, String> {
    ws::delete_specific_schedule_data(date).await
}

#[tauri::command]
async fn get_work_schedule_data() -> Result<Vec<WorkRecord>, String> {
    ws::get_work_schedule_data().await
}

//===============WORKSCHEDULEDEFAULTVALUE=================================
#[tauri::command]
async fn init_default_value_db() -> bool {
    if init_pool().await.is_err() {
        return false;
    }
    sdv::init_default_value_db().await.is_ok()
}

#[tauri::command]
async fn update_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    sdv::update_default_work_schedule(props).await
}

#[tauri::command]
async fn get_default_work_schedule() -> Result<Option<WorkRecord>, String> {
    sdv::get_default_work_schedule().await
}

//=============Payroll========================
#[tauri::command]
async fn calc_total_salary(
    work_data: Vec<WorkRecord>,
    selected_date: NaiveDate,
) -> Result<i32, String> {
    payroll::calc_total_salary(&work_data, &selected_date)
}

//===============CSV=========================================
#[tauri::command]
async fn import_csv(path: String) -> Result<bool, String> {
    println!("Importing CSV from path: {}", path);
    let result: Result<bool, String> = match csv::import_csv::import_csv(path).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    };
    result
}

//===============CORE=========================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // DB 初期化 & パス表示
            if let Err(e) = tauri::async_runtime::block_on(async { crate::init_pool().await }) {
                eprintln!("[DB] init failed: {e}");
            } else if let Some(p) = resolved_db_path() {
                println!("[DB] using path: {}", p.display());
                // 確認用にウィンドウイベントで通知も可能（必要なら）
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.emit("db_path", p.display().to_string());

                    let win_cloned: tauri::WebviewWindow = win.clone();

                    win.on_window_event(move |e: &WindowEvent| {
                        if let WindowEvent::DragDrop(ev) = e {
                            if let DragDropEvent::Drop { paths, .. } = ev {
                                let dropped_files: Vec<shared_types::DroppedFile> = paths
                                    .iter()
                                    .filter_map(|p| {
                                        let name: String =
                                            p.file_name()?.to_string_lossy().into_owned();

                                        let ext: String = p
                                            .extension()
                                            .and_then(|s| s.to_str())
                                            .unwrap_or("")
                                            .to_lowercase();

                                        let size: u64 =
                                            fs::metadata(p).map(|m| m.len()).unwrap_or(0);

                                        Some(shared_types::DroppedFile {
                                            path: p.to_string_lossy().into_owned(),
                                            name,
                                            ext,
                                            size,
                                        })
                                    })
                                    .collect();

                                win_cloned
                                    .emit("file_dropped", dropped_files)
                                    .unwrap_or_else(|e| {
                                        eprintln!("[DragDrop] emit error: {e}");
                                    });
                            }
                        }
                    });
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_cash_flows,
            add_cash_flow,
            init_db,
            delete_whole_data,
            init_work_schedule_db,
            add_work_schedule,
            init_default_value_db,
            update_default_work_schedule,
            delete_work_schedule_data,
            get_default_work_schedule,
            delete_specific_data,
            get_work_schedule_data,
            db_path,
            delete_specific_schedule_data,
            update_work_schedule,
            calc_total_salary,
            import_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
