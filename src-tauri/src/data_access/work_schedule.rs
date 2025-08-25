use crate::data_access::pool;
use shared_types::WorkRecord;
use sqlx::{Executor, Row};

pub async fn init_db() -> Result<(), String> {
    let ddl = r#"
        CREATE TABLE IF NOT EXISTS work_schedule (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            date        TEXT    NOT NULL,
            start_time  TEXT    NOT NULL,
            end_time    TEXT    NOT NULL,
            hourly_wage INTEGER NOT NULL,
            rest_time   TEXT    NOT NULL,
            minutes     INTEGER NOT NULL,
            amount      INTEGER NOT NULL,
            note        TEXT
        );
    "#;
    pool()
        .execute(sqlx::query(ddl))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn add_work_schedule(props: WorkRecord) -> Result<bool, String> {
    sqlx::query("INSERT INTO work_schedule (date, start_time, end_time, rest_time, hourly_wage, minutes, amount, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")
        .bind(props.date)
        .bind(props.start_time)
        .bind(props.end_time)
        .bind(props.rest_time)
        .bind(props.hourly_wage)
        .bind(props.minutes)
        .bind(props.amount)
        .bind(props.note)
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

pub async fn update_work_schedule(date: String, props: WorkRecord) -> Result<bool, String> {
    sqlx::query("UPDATE work_schedule SET start_time = ?1, end_time = ?2, rest_time = ?3, hourly_wage = ?4, minutes = ?5, amount = ?6, note = ?7 WHERE date = ?8")
        .bind(props.start_time)
        .bind(props.end_time)
        .bind(props.rest_time)
        .bind(props.hourly_wage)
        .bind(props.minutes)
        .bind(props.amount)
        .bind(props.note)
        .bind(date)
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

pub async fn get_work_schedule_data() -> Result<Vec<WorkRecord>, String> {
    let rows: Vec<sqlx::sqlite::SqliteRow> = match sqlx::query("SELECT * FROM work_schedule")
        .fetch_all(pool())
        .await
    {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let mut work_records: Vec<WorkRecord> = Vec::new();

    if rows.is_empty() {
        return Ok(work_records);
    }

    for row in rows {
        work_records.push(WorkRecord {
            date: row.get("date"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            hourly_wage: row.get("hourly_wage"),
            rest_time: row.get("rest_time"),
            minutes: row.get("minutes"),
            amount: row.get("amount"),
            note: row.get("note"),
        });
    }
    Ok(work_records)
}

pub async fn delete_work_schedule_data() -> Result<(), String> {
    sqlx::query("DELETE FROM work_schedule")
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_specific_schedule_data(date: String) -> Result<bool, String> {
    sqlx::query("DELETE FROM work_schedule WHERE date = ?1")
        .bind(date)
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}
