use shared_types::WorkRecord;
use crate::data_access::pool;
use sqlx::Executor;

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
    pool().execute(sqlx::query(ddl)).await.map_err(|e| e.to_string())?;
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

pub async fn delete_work_schedule_data() -> Result<(), String> {
    sqlx::query("DELETE FROM work_schedule")
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
