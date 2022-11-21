use super::*;
use chrono::Datelike;
use chrono::{DateTime, Duration, Utc};

#[derive(PartialEq, Clone, Debug)]
pub enum WeddingDayStatus {
    Coming,
    Today,
    Passed,
}

#[derive(Properties, PartialEq)]
pub struct WeddingDayProviderProps {
    #[prop_or_default]
    pub children: Children,
    pub wedding_datetime: DateTime<Utc>,
}

#[derive(PartialEq, Clone)]
pub struct WeddingDayInfo {
    pub relative_day_status: WeddingDayStatus,
    pub datetime_str: String,
}

#[function_component(WeddingDayProvider)]
pub fn wedding_day_provider(props: &WeddingDayProviderProps) -> Html {
    let now = Utc::now();
    let time_diff = now.signed_duration_since(props.wedding_datetime);
    if time_diff.num_days() > 1 {}

    html! {
        <ContextProvider<WeddingDayInfo> context={
            WeddingDayInfo{
                relative_day_status: WeddingDayStatus::Coming,
                datetime_str: "Something here".into(),
            }
        }>
            {for props.children.iter()}
        </ContextProvider<WeddingDayInfo>>
    }
}

pub fn get_wedding_day_status(
    wedding_day: &DateTime<Utc>,
    now: &DateTime<Utc>,
    offset: i32,
) -> WeddingDayStatus {
    let wedding_day_with_offset = *wedding_day + Duration::seconds(offset.into());
    let now_with_offset = *now + Duration::seconds(offset.into());

    if wedding_day_with_offset.day() == now_with_offset.day() {
        return WeddingDayStatus::Today;
    }

    if wedding_day_with_offset.timestamp_millis() > now_with_offset.timestamp_millis() {
        return WeddingDayStatus::Coming;
    }
    WeddingDayStatus::Passed
}

#[test]
fn wedding_day_status_should_be_correct() {
    use chrono::TimeZone;
    let wedding_day = Utc.with_ymd_and_hms(2024, 12, 25, 22, 0, 0).unwrap();
    let offset = 10 * 3600;

    let check_day_today = Utc.with_ymd_and_hms(2024, 12, 25, 15, 0, 0).unwrap();
    let check_day_coming = Utc.with_ymd_and_hms(2024, 12, 25, 13, 0, 0).unwrap();
    let check_day_today_2 = Utc.with_ymd_and_hms(2024, 12, 26, 13, 0, 0).unwrap();
    let check_day_passed = Utc.with_ymd_and_hms(2024, 12, 26, 15, 0, 0).unwrap();

    let today = get_wedding_day_status(&wedding_day, &check_day_today, offset);
    assert_eq!(today, WeddingDayStatus::Today);

    let today = get_wedding_day_status(&wedding_day, &check_day_today_2, offset);
    assert_eq!(today, WeddingDayStatus::Today);

    let coming = get_wedding_day_status(&wedding_day, &check_day_coming, offset);
    assert_eq!(coming, WeddingDayStatus::Coming);

    let passed = get_wedding_day_status(&wedding_day, &check_day_passed, offset);
    assert_eq!(passed, WeddingDayStatus::Passed);
}
