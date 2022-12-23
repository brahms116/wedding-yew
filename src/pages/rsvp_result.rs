use super::*;

mod controller;
mod copy;
mod state;

use controller::*;
use copy::*;
use state::*;

#[function_component(RsvpResultPage)]
pub fn rsvp_result() -> Html {
    let state = use_state(RsvpResultState::default);

    let invitation_service =
        use_context::<InvitationCtxValue>().expect("Try providing a invitation context");

    let (items, default_route) = use_auth();

    let controller = {
        let invitation_service = invitation_service.clone();
        let state = state.clone();
        RsvpResultController {
            invitation_service,
            state_setter: state,
        }
    };

    {
        let controller = controller.clone();
        let invitation_service = invitation_service.clone();
        use_effect_with_deps(
            move |_| {
                controller.on_fetch_invite_handle_change();
            },
            vec![invitation_service.fetch_invite_handle().clone()],
        )
    }
    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={items}/>
            <div
                id={"rsvp-result-text"}
                class={format!("
                    w-full h-[100vh] flex justify-center items-center {}
                ",(*state).clone().loading_css_class)}>
                {(*state).clone().title_text}
            </div>
        </div>
    }
}
