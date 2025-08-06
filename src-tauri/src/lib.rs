mod data_access;

use shared_types::{CashFlow, AddCashFlowProps};
use data_access::data_access as dac;

#[tauri::command]
async fn init_db() -> bool {
    match dac::init_db().await {
        Ok(_) => true,
        Err(_) => {
            false
        }
    }
}

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
async fn delete_whole_data() -> Result<bool, String> {
  dac::delete_whole_data().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_cash_flows,
            add_cash_flow,
            init_db,
            delete_whole_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
