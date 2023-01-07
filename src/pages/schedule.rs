use super::*;

#[function_component(SchedulePage)]
pub fn schedule_page() -> Html {
    let (nav_items, default_route) = use_auth();
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
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"2-3pm: Ceremony - Ann Street Presbyterian Church"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"3-4pm: Afternoon tea and photos - Church Foyer"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"5-10pm: Reception - Transcontinental Hotel"}
                </div>
            </div>
        </div>
    }
}
