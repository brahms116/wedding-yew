use super::*;
use chrono::{DateTime, Utc};

#[derive(PartialEq, Clone)]
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
