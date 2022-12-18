use super::*;

#[function_component(StoryPage)]
pub fn story_page() -> Html {
    let (nav_items, default_route) = use_auth();
    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={nav_items}/>
            <div class={"flex flex-col my-[130px] px-[32px] max-w-[100%] md:px-[132px]"}>
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"Our Story"}
                </div>
            </div>
        </div>
    }
}
