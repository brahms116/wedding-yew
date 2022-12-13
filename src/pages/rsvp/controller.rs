use super::*;
use tracing::{debug, error};

type A<T, E> = AsyncResourceHandle<T, E>;

#[derive(Clone)]
pub struct RsvpPageController<D, R, P>
where
    D: 'static + Clone + Dispatch<RsvpStateAction>,
    R: 'static + Clone + InvitationService,
    P: 'static + Clone + RoutePage<Route>,
{
    pub current_state: RsvpState,
    pub dispatch: D,
    // TODO: Add wedding service here
    pub router: P,
    pub invitation_service: R,
    pub id: Option<String>,
}

impl<D, R, P> RsvpPageController<D, R, P>
where
    D: 'static + Clone + Dispatch<RsvpStateAction>,
    R: 'static + Clone + InvitationService,
    P: 'static + Clone + RoutePage<Route>,
{
    fn handle_invite_data(&self, data: &InviteInfo) {
        self.dispatch.send(RsvpStateAction::Loading(false));
        if let Some(invite) = &data.invite {
            self.dispatch
                .send(RsvpStateAction::FormUpdate(invite.clone()))
        } else {
            self.router.goto(Route::Landing, None)
        }
    }

    pub fn on_fetch_response_change(&self) {
        let invite_handle = self.invitation_service.fetch_invite_handle();
        self.dispatch.send(RsvpStateAction::Loading(false));
        if let A::Success(d) = invite_handle {
            self.handle_invite_data(d)
        }
    }

    pub fn on_form_update(&self, form_data: &Invitation) {
        debug!(form_update=?form_data);
        self.dispatch
            .send(RsvpStateAction::FormUpdate(form_data.clone()))
    }

    pub fn on_form_submit(&self) {
        debug!(submitted_invitation = ?self.current_state.invitation);
        let save_response_handle = self.invitation_service.rsvp_handle();
        match save_response_handle {
            A::None | A::InitialErr(..) | A::SubsequentErr(..) | A::Success(..) => {
                self.dispatch.send(RsvpStateAction::SubmitLoading(true));
                self.invitation_service
                    .rsvp(&self.current_state.invitation.clone());
            }
            A::SubsequentLoad(..) | A::InitialLoad => return,
        }
    }

    pub fn on_submit_end(&self) {
        self.dispatch.send(RsvpStateAction::SubmitLoading(false));
        let save_response_handle = self.invitation_service.rsvp_handle();
        match save_response_handle {
            A::SubsequentErr(e, ..) | A::InitialErr(e) => {
                error!("{}", e)
            }
            A::Success(..) => {
                if let Some(id) = self.id.as_ref() {
                    self.invitation_service.fetch_invite(id);
                }
                self.router.goto(Route::RSVPResult, self.id.clone())
            }
            _ => return,
        }
    }

    pub fn on_dismount(&self) {
        self.invitation_service.reset_rsvp_handle();
    }
}
