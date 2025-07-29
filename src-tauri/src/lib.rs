mod entity;

use once_cell::sync::Lazy;
use sea_orm::{Database, DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use crate::entity::cash_flow;

static DB: Lazy<DatabaseConnection> = Lazy::new(|| {
    tokio::runtime::Handle::current()
        .block_on(Database::connect("sqlite://xxus.db?mode=rwc"))
        .expect("Cannot connect DB")
});

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
async fn list_cash_flows() -> Result<Vec<cash_flow::Model>, String> {
    cash_flow::Entity::find()
        .all(&*DB)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
async fn add_cash_flow(amount: i32, name: String, flow: String) -> Result<(), String> {
    let model = cash_flow::ActiveModel {
        amount: Set(amount.to_string()),
        name: Set(name),
        flow: Set(flow),
        ..Default::default()
    };
    model.insert(&*DB).await.map(|_| ()).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_cash_flows, add_cash_flow])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
