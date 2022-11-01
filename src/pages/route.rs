use super::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Landing,
    #[at("/rsvp")]
    RSVP,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Landing => html! {<landing::LandingPage/>},
        Route::RSVP => html! {<rsvp::RSVPPage/>},
    }
}
