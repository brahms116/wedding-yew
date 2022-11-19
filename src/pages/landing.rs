mod splash;
mod title;

use super::*;
use title::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="bg-bg">
            <NavMenu<Route> routes={vec![
                (Route::RSVP,"RSVP".to_owned()),
                (Route::Landing,"FAQ".to_owned()),
                (Route::RSVP,"GIFTS".to_owned()),
            ]}/>
            <Title/>
        </div>
    }
}
