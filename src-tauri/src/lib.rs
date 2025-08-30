mod data_access;

use data_access::{
    data_access as dac, init_pool, setting_default_value as sdv, work_schedule as ws,
};
use shared_types::{AddCashFlowProps, CashFlow, WorkRecord};
use crate::data_access::db_path as resolved_db_path;
use tauri::{Manager, Emitter};
use payroll_core as payroll;
use chrono::NaiveDate;

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
async fn calc_total_salary(work_data: &Vec<WorkRecord>, selected_date: &NaiveDate) -> Result<i32, String> {
    payroll::calc_total_salary(work_data, selected_date)
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
            calc_total_salary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
