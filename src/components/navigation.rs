use gloo_utils::document;
use web_sys::HtmlElement;

use super::*;

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    let menu_open = use_state(|| false);

    let desktop_links = "
        md:flex text-sm text-bold hidden
    ";

    let menu_icon = "
        md:hidden hover:text-slate-400 cursor-pointer
    ";

    let mobile_menu = "
        md:hidden fixed top-0 left-0 bg-white min-h-screen w-full
        flex flex-col justify-center items-center text-sm text-bold
        transition-transform
    ";

    let menu_style = if *menu_open {
        "transform: translateX(0%)"
    } else {
        "transform: translateX(100%)"
    };

    let cb = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| {
            menu_open.set(!(*menu_open));
        })
    };

    {
        let menu_open_dep = menu_open.clone();
        use_effect_with_deps(
            move |_| {
                let body: HtmlElement = document().body().unwrap();
                if *menu_open {
                    body.style().set_property("overflow-y", "hidden").unwrap();
                } else {
                    body.style().set_property("overflow-y", "auto").unwrap();
                }

                || {
                    document()
                        .body()
                        .unwrap()
                        .style()
                        .set_property("overflow-y", "auto")
                        .unwrap();
                }
            },
            *menu_open_dep,
        );
    }

    html! {
        <>
            <div class={"
                    fixed top-0 w-full left-0 p-8 flex justify-between h-16 items-center
                    bg-white z-10
                "}
            >
                <div class={"text-xl cursor-pointer"}>
                    {"Mia & David"}
                </div>
                <div class={desktop_links}>
                    <div class={"ml-8 cursor-pointer"}>{"FAQ"}</div>
                    <div class={"ml-8 cursor-pointer"}>{"RSVP"}</div>
                    <div class={"ml-8 cursor-pointer"}>{"GIFTS"}</div>
                </div>
                <div class={menu_icon} onclick={cb}>
                    <i class={"fa-solid fa-bars"}/>
                </div>
            </div>
            <div class={mobile_menu} style={menu_style}>
                <div class={"mb-8 cursor-pointer"}>{"FAQ"}</div>
                <div class={"mb-8 cursor-pointer"}>{"RSVP"}</div>
                <div class={"mb-8 cursor-pointer"}>{"GIFTS"}</div>
            </div>
        </>
    }
}
