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
    #[at("/story")]
    Story,
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
        Route::FAQ => html! {<rsvp::RSVPPage/>},
        Route::Story => html! {<rsvp::RSVPPage/>},
    }
}
