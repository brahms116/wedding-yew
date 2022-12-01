use super::*;

pub enum LandingStateAction {
    Loading,
    AcceptSplash,
    Today(String, String),
    Coming(String),
    Passed(String),
    TodayInvited(String, String, Invitation),
    ComingInvited(String, Invitation),
    PassInvited(String, Invitation),
}

#[derive(Default)]
pub struct LandingState {
    pub enter_button_loading: bool,
    pub splash_accepted: bool,
    pub cta_button_text: String,
    pub cta_button_route: NavDestination<Route>,
    pub nav_menu_items: Vec<(NavDestination<Route>, String)>,
    pub title_text: String,
    pub subtitle_text: String,
}

impl LandingState {
    pub fn loading(&mut self) {
        self.enter_button_loading = true;
        self.splash_accepted = false;
    }
}

impl Reducible for LandingState {
    type Action = LandingStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        todo!()
    }
}

#[cfg(test)]
mod landing_state_test {
    use super::*;

    #[test]
    fn should_load() {
        let state = LandingState::default();
        let state = Reducible::reduce(std::rc::Rc::new(state), LandingStateAction::Loading);
        assert_eq!(state.enter_button_loading, true);
        assert_eq!(state.splash_accepted, false);
    }

    #[test]
    fn should_today() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::Today("www.google.com".into(), "abc".into()),
        );

        let items = vec![
            (
                NavDestination::External(String::from("www.google.com")),
                String::from("Live Stream"),
            ),
            (NavDestination::App(Route::FAQ), String::from("FAQ")),
        ];

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.cta_button_text, "Live Stream".to_owned());
        assert_eq!(
            state.cta_button_route,
            NavDestination::External(String::from("www.google.com"))
        );
        assert_eq!(state.nav_menu_items, items);
        assert_eq!(state.title_text, get_today_title());
        assert_eq!(state.subtitle_text, get_today_subtitle())
    }
}
