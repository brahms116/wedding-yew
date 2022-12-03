use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};

pub fn get_utc_offset() -> Duration {
    Duration::seconds(10 * 3600)
}

pub fn get_wedding_day() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2022, 12, 3, 1, 0, 0).unwrap() - get_utc_offset()
}

pub fn get_live_stream_url() -> String {
    String::from("https://www.google.com")
}
