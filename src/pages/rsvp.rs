mod controller;
mod form;
mod state;
use super::*;
use controller::*;
use form::*;
use state::*;
use tracing::{debug, info};

#[function_component(RSVPPage)]
pub fn rsvp_page() -> Html {
    let state = use_reducer(RsvpState::default);
    let user_id = use_query_id();
    let invitation_service =
        use_context::<InvitationCtxValue>().expect("Invitation service is missing");
    let router = use_navigator().expect("Navigator is missing");
    let (nav_items, default_route) = use_auth();
    let controller = {
        let user_id = user_id.clone();
        let invitation_service = invitation_service.clone();
        RsvpPageController {
            current_state: (*state).clone(),
            dispatch: state.clone(),
            router,
            invitation_service,
            id: user_id,
        }
    };

    {
        let controller = controller.clone();
        use_effect_with_deps(
            move |_| {
                let controller = controller.clone();
                move || {
                    controller.on_dismount();
                }
            },
            (),
        )
    }

    {
        let controller = controller.clone();
        let dep = invitation_service.rsvp_handle().clone();
        use_effect_with_deps(
            move |_| {
                info!("rsvp page calling on_submit_end");
                controller.on_rsvp_handle_change();
                || {}
            },
            vec![dep],
        )
    }
    {
        let controller = controller.clone();
        let dep = invitation_service.rsvp_handle().clone();
        use_effect_with_deps(
            move |_| {
                info!("rsvp page calling on_fetch_end");
                controller.on_fetch_invite_handle_change();
                || {}
            },
            vec![dep],
        )
    }

    let on_change_cb = {
        //TODO: move logic inside controller
        let form_data = state.invitation.clone();
        let controller = controller.clone();
        Callback::from(move |invitee: Invitee| {
            let mut new_form_data = form_data.clone();
            if invitee.id == new_form_data.primary_invitee.id {
                new_form_data.primary_invitee = invitee;
            } else {
                let index = new_form_data
                    .dependents
                    .iter()
                    .position(|e| e.id == invitee.id);
                if let Some(i) = index {
                    new_form_data.dependents[i] = invitee;
                }
            }
            controller.on_form_update(&new_form_data)
        })
    };

    let on_submit_click = {
        let controller = controller.clone();
        Callback::from(move |_: MouseEvent| {
            controller.on_form_submit();
        })
    };

    let submit_button_text = if state.is_submit_loading {
        "Loading..."
    } else {
        "Submit"
    };

    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={nav_items}/>
            <div class={"flex flex-col my-[130px] px-[32px] max-w-[100%] md:px-[132px]"}>
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"RSVP"}
                </div>
                if !state.is_invite_loading {
                    <Form
                        on_change={
                            on_change_cb.clone()
                        }
                        invitee={state.invitation.primary_invitee.clone()}
                    />
                    {
                        state.invitation.dependents.iter().map(|e| {
                            let on_change_cb = on_change_cb.clone();
                            html!{
                                <Form
                                    key={e.clone().id}
                                    invitee={e.clone()}
                                    on_change={on_change_cb}
                                />
                        }}).collect::<Html>()
                    }
                    <div class="h-[48px]"/>
                    <div>
                        <button
                        type="button"
                        class="
                            p-2 bg-black text-white w-36
                            rounded-full
                        "
                        onclick={on_submit_click}
                        >{"SUBMIT"}</button>
                    </div>
                }
                else {
                    <div>{submit_button_text}</div>
                }
            </div>
        </div>
    }
}
