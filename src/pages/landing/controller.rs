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

#[cfg(test)]
mod landing_controller_tests {
    use super::*;
}

