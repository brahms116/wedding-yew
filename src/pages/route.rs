use super::*;

/// All of the app's possible routes expressed as an enum
#[derive(Clone, Routable, PartialEq, Eq, Hash)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Landing,
    #[at("/rsvp")]
    RSVP,
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
    }
}
