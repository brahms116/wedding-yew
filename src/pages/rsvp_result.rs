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

    let items = use_auth();

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
                controller.on_invite_data_change();
            },
            vec![invitation_service.invite_data().clone()],
        )
    }
    html! {
        <div>
            <NavMenu<Route, UrlQuery> routes={items}/>
            <div
                class="
                    w-full h-[100vh] flex justify-center items-center
                ">
                {(*state).clone().title_text}
            </div>
        </div>
    }
}
