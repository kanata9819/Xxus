use rusqlite::{Connection, Result};
use shared_types::WorkRecord;

const DB_NAME: &str = "xxus.db";

pub async fn init_default_value_db() -> Result<Connection> {
    let conn: Connection = Connection::open(DB_NAME)?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS  work_schedule_default_values(
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            date        TEXT    NOT NULL,
            start_time  TEXT    NOT NULL,
            end_time    TEXT    NOT NULL,
            hourly_wage NUMBER  NOT NULL,
            rest_time   TEXT    NOT NULL,
            minutes     NUMBER  NOT NULL,
            amount      NUMBER  NOT NULL,
            note        TEXT
        );
    "#,
    )?;
    Ok(conn)
}

pub async fn get_default_work_schedule() -> Result<WorkRecord, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT * FROM work_schedule_default_values LIMIT 1").map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

    if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        Ok(WorkRecord {
            date: row.get(1).map_err(|e| e.to_string())?,
            start_time: row.get(2).map_err(|e| e.to_string())?,
            end_time: row.get(3).map_err(|e| e.to_string())?,
            rest_time: row.get(4).map_err(|e| e.to_string())?,
            hourly_wage: row.get(5).map_err(|e| e.to_string())?,
            minutes: row.get(6).map_err(|e| e.to_string())?,
            amount: row.get(7).map_err(|e| e.to_string())?,
            note: row.get(8).map_err(|e| e.to_string())?,
        })
    } else {
        Err("No default work schedule found".to_string())
    }
}

pub async fn update_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    delete_default_work_schedule().await.map_err(|e| e.to_string())?;
    add_default_work_schedule(props).await.map_err(|e| e.to_string())?;
    Ok(true)
}

async fn add_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO work_schedule_default_values (date, start_time, end_time, rest_time, hourly_wage, minutes, amount, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![props.date, props.start_time, props.end_time, props.rest_time, props.hourly_wage, props.minutes, props.amount, props.note],
    )
    .map_err(|e| e.to_string())?;
    Ok(true)
}

async fn delete_default_work_schedule() -> Result<bool, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM work_schedule_default_values", [])
        .map_err(|e| e.to_string())?;
    Ok(true)
}
