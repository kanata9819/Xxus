use crate::data_access::pool;
use chrono::Local;
use shared_types::{AddCashFlowProps, CashFlow};
use sqlx::{Executor, Row};

pub async fn init_db() -> Result<(), String> {
    // Ensure pool initialized outside (lib.rs) before calling.
    let ddl: &'static str = r#"
        CREATE TABLE IF NOT EXISTS cash_flow (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            amount      INTEGER NOT NULL,
            name        TEXT    NOT NULL,
            flow        TEXT    NOT NULL,
            created_at  TEXT    NOT NULL
        );
    "#;
    pool()
        .execute(sqlx::query(ddl))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_cash_flows() -> Result<Vec<CashFlow>, String> {
    let rows = sqlx::query("SELECT id, amount, name, flow, created_at FROM cash_flow")
        .fetch_all(pool())
        .await
        .map_err(|e| e.to_string())?;
    let data: Vec<CashFlow> = rows
        .into_iter()
        .map(|r| CashFlow {
            id: r.get::<i64, _>(0) as i32,
            amount: r.get::<i64, _>(1) as i32,
            name: r.get::<String, _>(2),
            flow: r.get::<String, _>(3),
            created_at: r.get::<String, _>(4),
        })
        .collect();
    Ok(data)
}

pub async fn add_cash_flow(props: AddCashFlowProps) -> Result<bool, String> {
    let date: String = Local::now().format("%Y-%m-%d").to_string();
    sqlx::query("INSERT INTO cash_flow (amount, name, flow, created_at) VALUES (?1, ?2, ?3, ?4)")
        .bind(props.amount)
        .bind(props.name)
        .bind(props.flow_type)
        .bind(date)
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

pub async fn delete_whole_data() -> Result<(), String> {
    sqlx::query("DELETE FROM cash_flow")
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_specific_data(id: i32) -> Result<bool, String> {
    sqlx::query("DELETE FROM cash_flow WHERE id = ?1")
        .bind(id)
        .execute(pool())
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| e.to_string())
}
