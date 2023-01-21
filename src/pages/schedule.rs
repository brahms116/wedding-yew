use super::*;

#[function_component(SchedulePage)]
pub fn schedule_page() -> Html {
    let (nav_items, default_route) = use_auth();
    let wedding_day_ctx =
        use_context::<WeddingDayCtxValue>().expect("Wedding day ctx should be provided");
    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={nav_items}/>
            <div class={"flex flex-col my-[130px] px-[32px] max-w-[100%] md:px-[132px]"}>
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"Schedule"}
                </div>
                <div
                    class={"text-[28px] mb-[8px] font-bold"}
                >
                {wedding_day_ctx.schedule_datetime_str}
                </div>
                <div
                    class={"text-[24px] mb-[6px] font-bold"}
                >
                {"2pm"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Ceremony - Ann Street Presbyterian Church"}
                </div>
                <div
                    class={"text-[24px] mb-[6px] font-bold"}
                >
                {"3pm"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Afternoon tea and photos - Church Foyer"}
                </div>
                <div
                    class={"text-[24px] mb-[6px] font-bold"}
                >
                {"5pm"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Reception - Transcontinental Hotel"}
                </div>
                <div
                    class={"text-[24px] mb-[6px] font-bold"}
                >
                {"10pm"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Reception ends"}
                </div>
            </div>
        </div>
    }
}
