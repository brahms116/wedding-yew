use super::*;

#[derive(Clone)]
pub struct RsvpPageController<D, R, P>
where
    D: 'static + Clone + Dispatch<RsvpStateAction>,
    R: 'static + Clone + ApiResource<InviteInfo, ApiError, String>,
    P: 'static + Clone + RoutePage<Route>,
{
    pub current_state: RsvpState,
    pub dispatch: D,
    pub router: P,
    pub invitation_resource: R,
}
