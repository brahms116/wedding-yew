#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

use super::*;

pub struct LandingPageController<D, R>
where
    D: 'static + Clone + Dispatch<LandingStateAction>,
    R: 'static + Clone + ApiResource<InviteInfo, ApiError, String>,
{
    pub current_state: LandingState,
    pub dispatch: D,
    pub wedding_day_info: WeddingDayInfo,
    pub invitation_resource: R,
}

impl<D, R> LandingPageController<D, R>
where
    D: 'static + Clone + Dispatch<LandingStateAction>,
    R: 'static + Clone + ApiResource<InviteInfo, ApiError, String>,
{
    pub fn init(&self) {
        self.dispatch.send(LandingStateAction::Loading)
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
    fn should_init() {
        let info = WeddingDayInfo {
            relative_day_status: WeddingDayStatus::Coming,
            datetime_str: String::default(),
        };

        let mut dispatch = MockObject::new();
        let resource = MockObject::new();

        dispatch
            .expect_send()
            .times(1)
            .with(predicate::eq(LandingStateAction::Loading))
            .return_const(());

        let controller = LandingPageController {
            current_state: LandingState::default(),
            dispatch,
            wedding_day_info: info,
            invitation_resource: resource,
        };
        controller.init();
    }
}
