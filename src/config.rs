use chrono::TimeZone;
use chrono::{DateTime, Utc};

pub fn get_wedding_day() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2023, 3, 10, 14, 0, 0).unwrap()
}
