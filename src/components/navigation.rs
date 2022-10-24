use yew::{function_component, html};

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    html! {
        <div class={"fixed top-0 w-full left-0 p-8 flex justify-between items-center"}>
            <div class={"text-xl"}>
                {"Mia & David"}
            </div>
            <div class={"flex text-sm text-bold"}>
                <div class={"ml-8 cursor-pointer"}>{"FAQ"}</div>
                <div class={"ml-8 cursor-pointer"}>{"RSVP"}</div>
                <div class={"ml-8 cursor-pointer"}>{"GIFTS"}</div>
            </div>
        </div>
    }
}
