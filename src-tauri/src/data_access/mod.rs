pub mod data_access;
pub mod work_schedule;
pub mod setting_default_value;
use std::{env, fs, path::PathBuf, str::FromStr};

use once_cell::sync::OnceCell;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_pool() -> Result<(), String> {
    if DB_POOL.get().is_some() { return Ok(()); }

    // 1) 環境変数優先
    let explicit_path = env::var("XXUS_DB_PATH").ok().map(PathBuf::from);

    // 2) 標準のユーザーデータディレクトリ
    let mut resolved_path = explicit_path
        .or_else(|| dirs::data_dir().map(|base| base.join("Xxus").join("xxus.db")))
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("xxus.db"));

    // 念のためパスを正規化（.. を潰す）
    if let Ok(canon) = resolved_path.canonicalize() {
        // 既存ファイルがあれば正規化、無い場合は親だけ正規化
        resolved_path = canon;
    } else if let Some(parent) = resolved_path.parent() {
        if let Ok(parent_canon) = parent.canonicalize() {
            resolved_path = parent_canon.join(resolved_path.file_name().unwrap());
        }
    }

    // 親ディレクトリを必ず作る
    if let Some(parent) = resolved_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create DB directory {:?}: {}", parent, e))?;
    }

    // ★ URL 組み立て禁止。ConnectOptions に “ファイル名” をそのまま渡す
    let opts = SqliteConnectOptions::from_str("sqlite://:memory:")
        .map_err(|e| e.to_string())?
        .filename(&resolved_path)
        .create_if_missing(true)
        .read_only(false);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .map_err(|e| format!("DB connect failed ({}): {}", resolved_path.display(), e))?;

    DB_POOL.set(pool).map_err(|_| "Pool already set".to_string())?;
    Ok(())
}


pub fn pool() -> &'static SqlitePool {
	DB_POOL.get().expect("DB pool not initialized. Call init_pool() first.")
}
