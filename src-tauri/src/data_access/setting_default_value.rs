use crate::data_access::pool;
use shared_types::WorkRecord;
use sqlx::{Executor, Row};

pub async fn init_default_value_db() -> Result<(), String> {
    let ddl: &'static str = r#"
        CREATE TABLE IF NOT EXISTS work_schedule_default_values (
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

pub async fn get_default_work_schedule() -> Result<Option<WorkRecord>, String> {
    let row_opt: Option<sqlx::sqlite::SqliteRow> = match sqlx::query("SELECT date, start_time, end_time, hourly_wage, rest_time, minutes, amount, note FROM work_schedule_default_values LIMIT 1")
        .fetch_optional(pool())
        .await {
        Ok(row) => row,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(r) = row_opt {
        let record = WorkRecord {
            date: r.try_get::<String, _>(0).map_err(|e| e.to_string())?,
            start_time: r.try_get::<String, _>(1).map_err(|e| e.to_string())?,
            end_time: r.try_get::<String, _>(2).map_err(|e| e.to_string())?,
            hourly_wage: r.try_get::<i64, _>(3).map_err(|e| e.to_string())? as i32,
            rest_time: r.try_get::<String, _>(4).map_err(|e| e.to_string())?,
            minutes: r.try_get::<i64, _>(5).map_err(|e| e.to_string())? as i32,
            amount: r.try_get::<i64, _>(6).map_err(|e| e.to_string())? as i32,
            note: r.try_get::<String, _>(7).map_err(|e| e.to_string())?,
        };
        Ok(Some(record))
    } else {
        Ok(None)
    }
}

pub async fn update_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    delete_default_work_schedule().await?;
    add_default_work_schedule(props).await?;
    Ok(true)
}

async fn add_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    sqlx::query("INSERT INTO work_schedule_default_values (date, start_time, end_time, rest_time, hourly_wage, minutes, amount, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")
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

async fn delete_default_work_schedule() -> Result<bool, String> {
    sqlx::query("DELETE FROM work_schedule_default_values")
        .execute(pool())
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}
