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
    pub fn on_fetch_invite_handle_change(&self) {
        match self.invitation_service.fetch_invite_handle() {
            AsyncResourceHandle::None
            | AsyncResourceHandle::InitialErr(..)
            | AsyncResourceHandle::SubsequentErr(..) => self.state_setter.set(RsvpResultState {
                title_text: "No Information".into(),
                loading_css_class: "".to_string(),
            }),
            AsyncResourceHandle::InitialLoad | AsyncResourceHandle::SubsequentLoad(..) => {
                self.state_setter.set(RsvpResultState {
                    title_text: "Loading your rsvp result".into(),
                    loading_css_class: "loading animate-pulse".to_string(),
                })
            }
            AsyncResourceHandle::Success(d) => {
                if let Some(invite) = &d.invite {
                    if invite.is_coming() {
                        self.state_setter.set(RsvpResultState {
                            title_text: get_coming_message(),
                            loading_css_class: "".to_string(),
                        })
                    } else {
                        self.state_setter.set(RsvpResultState {
                            title_text: get_not_coming_message(),
                            loading_css_class: "".to_string(),
                        })
                    }
                } else {
                    self.state_setter.set({
                        RsvpResultState {
                            title_text: "".into(),
                            loading_css_class: "".to_string(),
                        }
                    })
                }
            }
        }
    }
}
