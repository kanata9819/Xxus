use chrono::{NaiveDate, Datelike};
use shared_types::WorkRecord;

pub fn calc_total_salary(work_data: &[WorkRecord], display: &NaiveDate) -> i32 {
    let mut total: i32 = 0;
    let display_ym: String = format!("{}{:02}", display.year(), display.month());

    for record in work_data.iter() {
        let record_ymd: NaiveDate = record
            .date
            .parse::<NaiveDate>()
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let record_ym: String = format!("{}{:02}", record_ymd.year(), record_ymd.month());

        if record_ym == display_ym {
            total += record.amount;
        }
    }

    total
}
