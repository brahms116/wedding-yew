use super::*;

#[derive(PartialEq, Debug)]
pub enum RsvpStateAction {
    Loading(bool),
    SubmitLoading(bool),
    FormUpdate(Invitation),
}

#[derive(Default, Clone, Debug)]
pub struct RsvpState {
    pub is_submit_loading: bool,
    pub is_invite_loading: bool,
    pub invitation: Invitation,
}

impl RsvpState {
    fn set_invite_loading(&mut self, loading: bool) {
        self.is_invite_loading = loading;
    }
    fn set_submit_loading(&mut self, loading: bool) {
        self.is_submit_loading = loading;
    }
    fn update_form(&mut self, form: Invitation) {
        self.invitation = form
    }
}

impl Reducible for RsvpState {
    type Action = RsvpStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut state = (*self).clone();
        match action {
            RsvpStateAction::Loading(p) => state.set_invite_loading(p),
            RsvpStateAction::SubmitLoading(p) => state.set_submit_loading(p),
            RsvpStateAction::FormUpdate(p) => state.update_form(p),
        }
        state.into()
    }
}
