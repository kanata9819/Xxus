mod data_access;

use data_access::{data_access as dac, work_schedule as ws, setting_default_value as sdv};
use shared_types::{AddCashFlowProps, CashFlow, WorkRecord};

#[tauri::command]
async fn init_db() -> bool {
    match dac::init_db().await {
        Ok(_) => true,
        Err(_) => false,
    }
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

//===============WORKSCHEDULE=================================
#[tauri::command]
async fn init_work_schedule_db() -> bool {
    match ws::init_db().await {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
async fn add_work_schedule(props: WorkRecord) -> Result<bool, String> {
  ws::add_work_schedule(props).await
}

#[tauri::command]
async fn delete_work_schedule_data( ) -> Result<(), String> {
    ws::delete_work_schedule_data().await
}

//===============WORKSCHEDULEDEFAULTVALUE=================================
#[tauri::command]
async fn init_default_value_db() -> bool {
  match sdv::init_default_value_db().await {
      Ok(_) => true,
      Err(_) => false,
  }
}

#[tauri::command]
async fn add_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    sdv::add_default_work_schedule(props).await
}

//===============CORE=========================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            add_default_work_schedule,
            delete_work_schedule_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
