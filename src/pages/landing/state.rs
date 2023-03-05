use super::*;

#[derive(PartialEq, Debug)]
pub enum LandingStateAction {
    Loading,
    AcceptSplash,
    Today(String, String),
    Coming(String),
    Passed(String),
    TodayInvited(String, String, Invitation),
    ComingInvited(String, Invitation, String),
    PassInvited(String, Invitation),
}

#[derive(Default, Clone, Debug)]
pub struct LandingState {
    pub enter_button_loading: bool,
    pub splash_accepted: bool,
    pub cta_button_text: String,
    pub cta_button_route: NavDestination<Route, UrlQuery>,
    pub title_text: String,
    pub cta_button_id: String,
    pub subtitle_text: String,
    pub wedding_date_time_text: String,
    pub rsvp_by_date: Option<String>,
}

impl LandingState {
    fn format_wedding_date_str(date_str: String) -> String {
        format!("{}", date_str)
    }
    pub fn loading(&mut self) {
        self.enter_button_loading = true;
        self.splash_accepted = false;
    }

    pub fn accept_splash(&mut self) {
        self.enter_button_loading = false;
        self.splash_accepted = true
    }

    pub fn today(&mut self, live_stream_url: String, wedding_date_str: String) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("Livestream");
        self.cta_button_id = String::from("live-stream-button");
        self.cta_button_route = NavDestination::External(String::from(live_stream_url));
        self.title_text = get_today_title();
        self.subtitle_text = get_today_subtitle();
    }

    pub fn today_invited(
        &mut self,
        live_stream_url: String,
        wedding_date_str: String,
        invite: Invitation,
    ) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("Livestream");
        self.cta_button_id = String::from("live-stream-button");
        self.cta_button_route = NavDestination::External(String::from(live_stream_url));
        self.title_text = get_today_invited_title(invite.get_fnames());
        self.subtitle_text = get_today_invited_subtitle();
    }

    pub fn coming(&mut self, wedding_date_str: String) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("Our Story");
        self.cta_button_id = String::from("our-story-button");
        self.cta_button_route = NavDestination::App(Route::Story);
        self.title_text = get_coming_title();
        self.subtitle_text = get_coming_subtitle();
    }

    pub fn coming_invited(
        &mut self,
        wedding_date_str: String,
        invite: Invitation,
        rsvp_by_datetime_str: String,
    ) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("RSVP");
        self.cta_button_id = String::from("rsvp-button");
        self.cta_button_route = NavDestination::AppWithQuery(
            Route::RSVP,
            UrlQuery {
                id: Some(invite.primary_invitee.id.clone()),
            },
        );
        self.title_text = get_coming_invited_title(invite.get_fnames());
        self.subtitle_text = get_coming_invited_subtitle();
        self.rsvp_by_date = Some(rsvp_by_datetime_str);
    }

    pub fn passed(&mut self, wedding_date_str: String) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("Our Story");
        self.cta_button_id = String::from("our-story-button");
        self.cta_button_route = NavDestination::App(Route::Story);
        self.title_text = get_passed_title();
        self.subtitle_text = get_passed_subtitle();
    }

    pub fn passed_invited(&mut self, wedding_date_str: String, invite: Invitation) {
        self.enter_button_loading = false;
        self.wedding_date_time_text = Self::format_wedding_date_str(wedding_date_str);
        self.cta_button_text = String::from("Our Story");
        self.cta_button_id = String::from("our-story-button");
        self.cta_button_route = NavDestination::AppWithQuery(
            Route::Story,
            UrlQuery {
                id: Some(invite.primary_invitee.id.clone()),
            },
        );
        self.title_text = get_passed_invited_title(invite.get_fnames());
        self.subtitle_text =
            get_passed_invited_subtitle(invite.primary_invitee.rsvp.unwrap_or(false));
    }
}

impl Reducible for LandingState {
    type Action = LandingStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut state = (*self).clone();
        match action {
            LandingStateAction::Loading => state.loading(),
            LandingStateAction::AcceptSplash => state.accept_splash(),
            LandingStateAction::Today(url, date_str) => state.today(url, date_str),
            LandingStateAction::Coming(date_str) => state.coming(date_str),
            LandingStateAction::Passed(date_str) => state.passed(date_str),
            LandingStateAction::TodayInvited(url, date_str, invite) => {
                state.today_invited(url, date_str, invite)
            }
            LandingStateAction::ComingInvited(date_str, invite, rsvp_by_date_str) => {
                state.coming_invited(date_str, invite, rsvp_by_date_str)
            }
            LandingStateAction::PassInvited(date_str, invite) => {
                state.passed_invited(date_str, invite)
            }
        };
        std::rc::Rc::new(state)
    }
}

#[cfg(test)]
mod landing_state_test {
    use super::*;

    fn invite() -> Invitation {
        Invitation {
            primary_invitee: Invitee {
                id: String::from("a"),
                fname: String::from("Joe"),
                lname: String::from("Smith"),
                rsvp: Some(false),
                dietary_requirements: String::default(),
            },
            dependents: vec![
                Invitee {
                    id: String::from("b"),
                    fname: String::from("Dane"),
                    lname: String::from("Smith"),
                    rsvp: Some(false),
                    dietary_requirements: String::default(),
                },
                Invitee {
                    id: String::from("c"),
                    fname: String::from("Jane"),
                    lname: String::from("Smith"),
                    rsvp: Some(false),
                    dietary_requirements: String::default(),
                },
            ],
        }
    }

    #[test]
    fn should_load() {
        let state = LandingState::default();
        let state = Reducible::reduce(std::rc::Rc::new(state), LandingStateAction::Loading);
        assert_eq!(state.enter_button_loading, true);
        assert_eq!(state.splash_accepted, false);
    }

    #[test]
    fn should_accept() {
        let state = LandingState::default();
        let state = Reducible::reduce(std::rc::Rc::new(state), LandingStateAction::AcceptSplash);
        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.splash_accepted, true);
    }

    #[test]
    fn should_today() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::Today("www.google.com".into(), "abc".into()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(state.cta_button_text, "Livestream".to_owned());
        assert_eq!(
            state.cta_button_route,
            NavDestination::External(String::from("www.google.com"))
        );
        assert_eq!(state.subtitle_text, get_today_subtitle())
    }

    #[test]
    fn should_today_invited() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::TodayInvited("www.google.com".into(), "abc".into(), invite()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(state.cta_button_text, "Livestream".to_owned());
        assert_eq!(
            state.cta_button_route,
            NavDestination::External(String::from("www.google.com"))
        );
        assert_eq!(
            state.title_text,
            get_today_invited_title(vec!["Joe".into(), "Dane".into(), "Jane".into()])
        );
        assert_eq!(state.subtitle_text, get_today_invited_subtitle())
    }

    #[test]
    fn should_coming() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::Coming("abc".into()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(state.title_text, get_coming_title());
        assert_eq!(state.subtitle_text, get_coming_subtitle())
    }

    #[test]
    fn should_coming_invited() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::ComingInvited("abc".into(), invite(), "abcd".to_string()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(state.cta_button_text, "RSVP".to_owned());
        assert_eq!(
            state.cta_button_route,
            NavDestination::AppWithQuery(
                Route::RSVP,
                UrlQuery {
                    id: Some("a".into())
                }
            ),
        );
        assert_eq!(
            state.title_text,
            get_coming_invited_title(vec!["Joe".into(), "Dane".into(), "Jane".into()])
        );
        assert_eq!(state.subtitle_text, get_coming_invited_subtitle())
    }

    #[test]
    fn should_passed_invited() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::PassInvited("abc".into(), invite()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(
            state.title_text,
            get_passed_invited_title(vec!["Joe".into(), "Dane".into(), "Jane".into()])
        );
        assert_eq!(state.subtitle_text, get_passed_invited_subtitle(false))
    }

    #[test]
    fn should_passed() {
        let state = LandingState::default();
        let state = Reducible::reduce(
            std::rc::Rc::new(state),
            LandingStateAction::Passed("abc".into()),
        );

        assert_eq!(state.enter_button_loading, false);
        assert_eq!(state.wedding_date_time_text, String::from("abc"));
        assert_eq!(state.title_text, get_passed_title());
        assert_eq!(state.subtitle_text, get_passed_subtitle())
    }
}
