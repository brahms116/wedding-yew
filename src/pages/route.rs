use super::*;

/// All of the app's possible routes expressed as an enum
#[derive(Clone, Debug, Routable, PartialEq, Eq, Hash)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Landing,
    #[at("/rsvp")]
    RSVP,
    #[at("/faq")]
    FAQ,
    #[at("/stream")]
    LiveStream,
    #[at("/story")]
    Story,
    #[at("/submit")]
    RSVPResult,
}

/// Switch function to pass into yew router to determine which component to render
///
/// # Arguements
/// * routes - An enum expressing all of the route possibilities of the app
///
/// # Returns
/// Html component to render
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Landing => html! {<landing::LandingPage/>},
        Route::RSVP => html! {<rsvp::RSVPPage/>},
        Route::FAQ => html! {<faq::FaqPage/>},
        Route::Story => html! {<story::StoryPage/>},
        Route::LiveStream => html! {<rsvp::RSVPPage/>},
        Route::RSVPResult => html! {<rsvp_result::RsvpResultPage/>},
    }
}
