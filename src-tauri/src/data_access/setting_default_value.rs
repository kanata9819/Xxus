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

pub async fn add_default_work_schedule(props: WorkRecord) -> Result<bool, String> {
    let conn = Connection::open(DB_NAME).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO work_schedule_default_values (date, start_time, end_time, rest_time, hourly_wage, minutes, amount, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![props.date, props.start_time, props.end_time, props.rest_time, props.hourly_wage, props.minutes, props.amount, props.note],
    )
    .map_err(|e| e.to_string())?;
    Ok(true)
}
