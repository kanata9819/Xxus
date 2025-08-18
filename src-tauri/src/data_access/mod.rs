pub mod data_access;
pub mod work_schedule;
pub mod setting_default_value;

use once_cell::sync::OnceCell;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_pool() -> Result<(), String> {
	if DB_POOL.get().is_some() { return Ok(()); }
	let pool = SqlitePoolOptions::new()
		.max_connections(5)
		.connect("sqlite:xxus.db")
		.await
		.map_err(|e| e.to_string())?;
	DB_POOL.set(pool).map_err(|_| "Pool already set".to_string())?;
	Ok(())
}

pub fn pool() -> &'static SqlitePool {
	DB_POOL.get().expect("DB pool not initialized. Call init_pool() first.")
}
