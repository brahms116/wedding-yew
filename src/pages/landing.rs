mod controller;
mod copy;
mod splash;
mod state;

use super::*;
use controller::*;
use copy::*;
use splash::*;
use state::*;
use web_sys::HtmlVideoElement;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let state = use_reducer(LandingState::default);
    let vid_ref = use_node_ref();
    let wedding_service =
        use_context::<WeddingDayCtxValue>().expect("Wedding service should be provided.");
    let invitation_service =
        use_context::<InvitationCtxValue>().expect("Invitation service should be provided");
    let navigator = use_navigator().expect("Navigator shoule exist");

    let live_stream_service =
        use_context::<LiveStreamService>().expect("Live stream service should be provided");

    let (nav_items, default) = use_auth();

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
            livesteam_url: live_stream_service.0.clone(),
        }
    };

    {
        let controller = controller.clone();
        let invitation_service_dep = invitation_service.clone();
        use_effect_with_deps(
            move |_| {
                controller.on_fetch_invite_handle_change();
                || {}
            },
            vec![invitation_service_dep.fetch_invite_handle().clone()],
        )
    }

    let on_click = {
        let vid_ref = vid_ref.clone();
        let controller = controller.clone();
        Callback::from(move |_: MouseEvent| {
            controller.on_splash_accepted();
            let element = vid_ref
                .cast::<HtmlVideoElement>()
                .expect("Ref should be video element");
            element.set_loop(true);
            element.play().unwrap();
        })
    };

    let on_cta_click = {
        let state = state.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            if let NavDestination::AppWithQuery(ref route, ref query) = state.cta_button_route {
                navigator.push_with_query(route, query).unwrap();
            }
            if let NavDestination::App(ref route) = state.cta_button_route {
                navigator.push(route);
            }
            if let NavDestination::External(ref url) = state.cta_button_route {
                web_sys::window()
                    .expect("Window should exist")
                    .location()
                    .assign(url)
                    .expect("Location should navigate");
            }
        })
    };

    {
        let state = state.clone();
        let vid_ref = vid_ref.clone();
        let on_cta_click = on_cta_click.clone();

        html! {
            <div class="bg-bg overflow-y-hidden">
                <NavMenu<Route, UrlQuery> default_route={default} routes={nav_items}/>
                <Splash
                    on_splash_click={on_click}
                    is_loading={state.enter_button_loading}
                />
                <ErrorDisplay/>
                <div class="
                    w-screen max-h-screen max-w-full overflow-y-auto
                ">
                    <div class="w-full flex justify-center">
                        <div class="
                           flex items-center flex-col text-center max-w-md
                           text-[1.125rem] overflow-x-hidden
                           mb-16 px-4 mt-20
                        ">
                            <div class="mb-4 italic">
                                {state.title_text.clone()}
                            </div>
                            <div class="italic">
                                {state.subtitle_text.clone()}
                            </div>
                            <div class="h-[300px]">
                                <video class="h-[200px] relative z-0" playsinline={true} ref={vid_ref}>
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
                                {state.wedding_date_time_text.clone()}
                            </div>
                            if state.rsvp_by_date.is_some() {
                                <div class="text-[1.125rem] mt-4 mb-4">
                                    {
                                        format!(
                                            "Please rsvp by - {}",
                                            state.rsvp_by_date.clone()
                                                .expect("Should have checked for none")
                                        )
                                    }
                                </div>
                            }
                            <div>
                                <button type="button"
                                    class="
                                        p-2 bg-black text-white w-36
                                        rounded-full
                                    "
                                    id={state.cta_button_id.clone()}
                                    onclick={on_cta_click}
                                >{state.cta_button_text.clone()}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
