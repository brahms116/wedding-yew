use yew::{function_component, html};
use yew_router::prelude::*;

mod components;
mod pages;

use components::WindowProvider;
use pages::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <WindowProvider>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>
        </WindowProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
