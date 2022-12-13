use super::*;

#[derive(Clone)]
pub struct RsvpResultController<I, S>
where
    I: InvitationService + Clone,
    S: SetState<RsvpResultState> + Clone,
{
    pub invitation_service: I,
    pub state_setter: S,
}

impl<I, S> RsvpResultController<I, S>
where
    I: InvitationService + Clone,
    S: SetState<RsvpResultState> + Clone,
{
    pub fn on_invite_data_change(&self) {
        match self.invitation_service.fetch_invite_handle() {
            AsyncResourceHandle::None
            | AsyncResourceHandle::InitialErr(..)
            | AsyncResourceHandle::SubsequentErr(..) => self.state_setter.set(RsvpResultState {
                title_text: "No Information".into(),
            }),
            AsyncResourceHandle::InitialLoad | AsyncResourceHandle::SubsequentLoad(..) => {
                self.state_setter.set(RsvpResultState {
                    title_text: "Loading...".into(),
                })
            }
            AsyncResourceHandle::Success(d) => {
                if let Some(invite) = &d.invite {
                    if invite.is_coming() {
                        self.state_setter.set(RsvpResultState {
                            title_text: get_coming_message(),
                        })
                    } else {
                        self.state_setter.set(RsvpResultState {
                            title_text: get_not_coming_message(),
                        })
                    }
                } else {
                    self.state_setter.set({
                        RsvpResultState {
                            title_text: "".into(),
                        }
                    })
                }
            }
        }
    }
}
