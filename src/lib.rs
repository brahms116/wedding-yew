mod components;
mod config;
pub mod pages;

use components::{FetchService, InviteProvider, LiveStreamServiceProvider, WeddingDayProvider};
use pages::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let fetch_service =
        FetchService("https://om179wgdqi.execute-api.ap-southeast-2.amazonaws.com/PROD/api".into());
    html! {
        <WeddingDayProvider
            utc_offset={config::get_utc_offset()}
            wedding_datetime={config::get_wedding_day()}
            rsvp_by_datetime={config::get_rsvp_by_date()}
        >
            <LiveStreamServiceProvider live_stream_url={config::get_live_stream_url()}>
                <InviteProvider<FetchService> api_service={fetch_service}>
                    <BrowserRouter>
                        <Switch<Route> render={switch}/>
                    </BrowserRouter>
                </InviteProvider<FetchService>>
            </LiveStreamServiceProvider>
        </WeddingDayProvider>
    }
}
