use super::*;
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;
use tracing::error;
type A<T, E> = AsyncResourceHandle<T, E>;

#[derive(Clone)]
pub struct LandingPageController<D, R>
where
    D: 'static + Clone + Dispatch<LandingStateAction>,
    R: 'static + Clone + InvitationService,
{
    pub current_state: LandingState,
    pub dispatch: D,
    pub wedding_day_info: WeddingDayInfo,
    pub invitation_resource: R,
    pub livesteam_url: String,
}

impl<D, R> LandingPageController<D, R>
where
    D: 'static + Clone + Dispatch<LandingStateAction>,
    R: 'static + Clone + InvitationService,
{
    fn handle_invite(&self, invite: Option<Invitation>) {
        if let Some(invite) = invite {
            match self.wedding_day_info.relative_day_status {
                WeddingDayStatus::Coming => self.dispatch.send(LandingStateAction::ComingInvited(
                    self.wedding_day_info.datetime_str.clone(),
                    invite,
                )),
                WeddingDayStatus::Today => self.dispatch.send(LandingStateAction::TodayInvited(
                    self.livesteam_url.clone(),
                    self.wedding_day_info.datetime_str.clone(),
                    invite,
                )),
                WeddingDayStatus::Passed => self.dispatch.send(LandingStateAction::PassInvited(
                    self.wedding_day_info.datetime_str.clone(),
                    invite,
                )),
            }
        } else {
            match self.wedding_day_info.relative_day_status {
                WeddingDayStatus::Coming => self.dispatch.send(LandingStateAction::Coming(
                    self.wedding_day_info.datetime_str.clone(),
                )),
                WeddingDayStatus::Today => self.dispatch.send(LandingStateAction::Today(
                    self.livesteam_url.clone(),
                    self.wedding_day_info.datetime_str.clone(),
                )),
                WeddingDayStatus::Passed => self.dispatch.send(LandingStateAction::Passed(
                    self.wedding_day_info.datetime_str.clone(),
                )),
            }
        }
    }

    fn handle_data(&self, data: &InviteInfo) {
        self.handle_invite(data.invite.clone());
    }

    pub fn on_accept(&self) {
        if !self.invitation_resource.invite_data().loading() {
            self.dispatch.send(LandingStateAction::AcceptSplash);
        }
    }

    pub fn on_fetch_response_change(&self) {
        match self.invitation_resource.invite_data() {
            A::Success(d) => self.handle_data(d),
            A::InitialErr(e) | A::SubsequentErr(e, ..) => {
                error!("{}", e);
                self.handle_invite(None)
            }
            A::InitialLoad | A::SubsequentLoad(..) => {
                self.dispatch.send(LandingStateAction::Loading)
            }
            A::None => self.handle_invite(None),
        }
    }
}

#[cfg(test)]
mod landing_controller_tests {
    use super::*;
    mock! {
        Object {}
        impl Dispatch<LandingStateAction> for Object {
            fn send(&self,action:LandingStateAction);
        }
        impl InvitationService for Object {
            fn invite_data(&self) -> &A<InviteInfo, ApiError>;
            fn fetch_invite(&self, id: &str);
            fn save_invite(&self, invite: &Invitation);
            fn save_response(&self) -> &A<bool, ApiError>;
            fn reset_save_request(&self);
        }
        impl Clone for Object {
            fn clone(&self)->Self;
        }
    }

    #[test]
    fn should_stale_init_with_id() {}

    #[test]
    fn should_init_without_id() {}
}
