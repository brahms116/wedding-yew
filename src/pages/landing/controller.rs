#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

use super::*;

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

    fn send_request(&self, id: &str) {
        self.dispatch.send(LandingStateAction::Loading);
        self.invitation_resource.fetch_invite(id);
    }

    fn handle_data(&self, data: &InviteInfo) {
        self.handle_invite(data.invite.clone());
    }

    pub fn init(&self, id: Option<&str>) {
        if let None = id {
            self.handle_invite(None);
            return;
        }
        let id = id.unwrap();
        let current_invite = self.invitation_resource.invite_data();
        match current_invite {
            AsyncResourceHandle::None => self.send_request(id),
            AsyncResourceHandle::SubsequentErr(.., d)
            | AsyncResourceHandle::Success(d)
            | AsyncResourceHandle::SubsequentLoad(d) => self.handle_data(d),
            // TODO: what about other states
            _ => {}
        };
    }

    pub fn on_accept(&self) {
        if !self.invitation_resource.invite_data().loading() {
            self.dispatch.send(LandingStateAction::AcceptSplash);
        }
    }

    pub fn on_fetch_end(&self) {
        if let AsyncResourceHandle::Success(d) = self.invitation_resource.invite_data() {
            self.handle_data(d)
        }
        if let AsyncResourceHandle::SubsequentErr(_, d) = self.invitation_resource.invite_data() {
            self.handle_data(d)
        }
        if let AsyncResourceHandle::InitialErr(_) = self.invitation_resource.invite_data() {
            self.handle_invite(None)
        }
        // TODO, if loading?
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
            fn invite_data(&self) -> &AsyncResourceHandle<InviteInfo, ApiError>;
            fn fetch_invite(&self, id: &str);
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

        resource
            .expect_invite_data()
            .times(1)
            .return_const(AsyncResourceHandle::None);
        resource
            .expect_fetch_invite()
            .with(predicate::eq(id))
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
