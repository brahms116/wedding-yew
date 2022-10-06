use yew::{function_component, html, Html};
use yew_router::prelude::*;

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Landing,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Landing => html! {<pages::landing::LandingPage/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
