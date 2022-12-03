mod controller;
mod copy;
mod splash;
mod state;

use super::*;
use controller::*;
use copy::*;
use splash::*;
use state::*;
use tracing::info;
use web_sys::HtmlVideoElement;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let state = use_reducer(LandingState::default);
    let user_id = use_query_id();
    let vid_ref = use_node_ref();
    let wedding_service =
        use_context::<WeddingDayInfo>().expect("Wedding service should be provided.");
    let invitation_service =
        use_context::<InviteProvidedInfo>().expect("Invitation service should be provided");
    let navigator = use_navigator().expect("Navigator shoule exist");

    let controller = {
        let dispatch = state.clone();
        let state = (*state).clone();
        let wedding_service = wedding_service.clone();
        let invitation_service = invitation_service.clone();
        LandingPageController {
            current_state: state,
            dispatch,
            wedding_day_info: wedding_service,
            invitation_resource: invitation_service,
            livesteam_url: "https://www.google.com".to_string(),
        }
    };

    {
        let id = user_id.clone();
        let controller = controller.clone();
        use_effect_with_deps(
            move |_| {
                info!("landing page calling init");
                controller.init(id.as_ref().map(|e| e.as_str()));
                || {}
            },
            (),
        )
    }

    {
        let controller = controller.clone();
        let invitation_service_dep = invitation_service.clone();
        let invitation_service = invitation_service.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(..) = invitation_service.data() {
                    info!("landing page calling on_fetch_end");
                    controller.on_fetch_end();
                }
                || {}
            },
            vec![invitation_service_dep.data()],
        )
    }

    let on_click = {
        let vid_ref = vid_ref.clone();
        let controller = controller.clone();
        Callback::from(move |_: MouseEvent| {
            controller.on_accept();
            let element = vid_ref
                .cast::<HtmlVideoElement>()
                .expect("Ref should be video element");
            element.set_loop(true);
            element.play().unwrap();
        })
    };

    let on_cta_click = {
        let state = state.clone();
        let id = use_query_id();
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            if let NavDestination::App(ref route) = state.cta_button_route {
                navigator
                    .push_with_query(route, &UrlQuery { id: id.clone() })
                    .unwrap();
            } else {
                web_sys::window()
                    .expect("Window should exist")
                    .location()
                    .assign("https://www.google.com")
                    .expect("Location should navigate");
            }
        })
    };

    {
        let state = state.clone();
        let vid_ref = vid_ref.clone();
        let on_cta_click = on_cta_click.clone();
        html! {
            <div class="bg-bg">
                <NavMenu<Route> routes={(*state).nav_menu_items.clone()}/>
                if !state.splash_accepted {
                    <Splash
                        on_splash_click={on_click}
                        is_loading={state.enter_button_loading}
                    />
                }
                <div class="
                mt-20 w-screen h-screen max-w-full flex 
                items-center justify-center
            ">
                    <div class="
                   flex items-center justify-center flex-col text-center max-w-md
                   text-[1.125rem] overflow-x-hidden
                   mb-16 px-4
                ">
                        <div class="mb-4 italic">
                            {state.title_text.clone()}
                        </div>
                        <div class="italic">
                            {state.subtitle_text.clone()}
                        </div>
                        <div class="h-[300px]">
                            <video class="h-[200px] relative z-0" ref={vid_ref}>
                                <source src="video.mp4" type="video/mp4"/>
                            </video>
                        </div>
                        <div class="text-5xl font-serif mb-6">
                            {"David & Mia"}
                        </div>
                        <div class="text-[1.125rem]">
                            {"Ann St Presbyterian"}
                        </div>
                        <div class="text-[1.125rem] mb-6">
                            {(*state).wedding_date_time_text.clone()}
                        </div>
                        <div>
                            <button type="button"
                                class="
                                p-2 bg-black text-white w-36
                                rounded-full
                            "
                                onclick={on_cta_click}
                            >{state.cta_button_text.clone()}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
