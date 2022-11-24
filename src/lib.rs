mod components;
mod config;
pub mod pages;

use components::WeddingDayProvider;
use pages::{switch, Route};
use yew::{function_component, html};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <WeddingDayProvider
            utc_offset={config::get_utc_offset()}
            wedding_datetime={config::get_wedding_day()}
        >
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>
        </WeddingDayProvider>
    }
}
