use chrono::Local;
use rusqlite::{Connection, Result};
use shared_types::{AddCashFlowProps, CashFlow};

const DB_NAME: &str = "xxus.db";

pub async fn init_db() -> Result<Connection> {
    let conn: Connection = Connection::open(DB_NAME)?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS cash_flow (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            amount  INTEGER NOT NULL,
            name    TEXT    NOT NULL,
            flow    TEXT    NOT NULL,
            date    TEXT    NOT NULL )
        );
    "#,
    )?;
    Ok(conn)
}

pub async fn list_cash_flows() -> Result<Vec<CashFlow>, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT * FROM cash_flow")
        .map_err(|e| e.to_string())?;

    let cash_flows_iter = stmt
        .query_map([], |row| {
            Ok(CashFlow {
                id: row.get(0)?,
                amount: row.get(1)?,
                name: row.get(2)?,
                flow: row.get(3)?,
                date: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(cash_flows_iter.filter_map(Result::ok).collect())
}

pub async fn add_cash_flow(props: AddCashFlowProps) -> Result<bool, String> {
    let date: chrono::DateTime<Local> = Local::now();
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO cash_flow (amount, name, flow, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![props.amount, props.name, props.flow_type, date.to_string()],
    )
    .map_err(|e| e.to_string())?;
    Ok(true)
}

pub async fn delete_whole_data() -> Result<bool, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM cash_flow", ()).map_err(|e| e.to_string())?;
    Ok(true)
}
