#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

use super::*;

#[derive(Clone)]
pub struct LandingPageController<D, R>
where
    D: 'static + Clone + Dispatch<LandingStateAction>,
    R: 'static + Clone + ApiResource<InviteInfo, ApiError, String>,
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
    R: 'static + Clone + ApiResource<InviteInfo, ApiError, String>,
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
    pub fn init(&self, id: Option<&str>) {
        if let None = id {
            self.handle_invite(None)
        } else if let Some(data) = self.invitation_resource.data() {
            self.handle_invite(data.invite)
        } else {
            let id = id.unwrap();
            self.dispatch.send(LandingStateAction::Loading);
            self.invitation_resource.fetch(id.to_string())
        }
    }

    pub fn on_accept(&self) {
        if !self.invitation_resource.loading() {
            self.dispatch.send(LandingStateAction::AcceptSplash);
        }
    }

    pub fn on_fetch_end(&self) {
        if let Some(data) = self.invitation_resource.data() {
            self.handle_invite(data.invite);
        } else {
            self.handle_invite(None);
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
        impl ApiResource<InviteInfo, ApiError, String> for Object {
            fn data(&self) -> Option<InviteInfo>;
            fn set_data(&self, setter: Box<dyn FnOnce(Option<InviteInfo>) -> Option<InviteInfo>>);
            fn fetch(&self, params: String);
            fn error(&self) -> Option<ApiError>;
            fn loading(&self) -> bool;
        }
        impl Clone for Object {
            fn clone(&self)->Self;
        }
    }

    #[test]
    fn should_fresh_init_with_id() {
        let info = WeddingDayInfo {
            relative_day_status: WeddingDayStatus::Coming,
            datetime_str: String::default(),
        };
        let mut dispatch = MockObject::new();
        let mut resource = MockObject::new();
        let id = "user_id";
        dispatch
            .expect_send()
            .times(1)
            .with(predicate::eq(LandingStateAction::Loading))
            .return_const(());

        resource.expect_data().times(1).return_const(None);
        resource
            .expect_fetch()
            .with(predicate::eq(String::from(id)))
            .times(1)
            .return_const(());

        let controller = LandingPageController {
            current_state: LandingState::default(),
            dispatch,
            wedding_day_info: info,
            invitation_resource: resource,
            livesteam_url: String::from("livestream_url"),
        };
        controller.init(Some(id));
    }

    #[test]
    fn should_stale_init_with_id() {}

    #[test]
    fn should_init_without_id() {}
}
