use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LiveStreamService(pub String);

#[derive(Properties, PartialEq)]
pub struct LiveStreamServiceProviderProps {
    pub children: Children,
    pub live_stream_url: String,
}

#[function_component(LiveStreamServiceProvider)]
pub fn live_stream_service_provider(props: &LiveStreamServiceProviderProps) -> Html {
    html! {
        <ContextProvider<LiveStreamService>
            context={LiveStreamService(props.live_stream_url.clone())}
        >
            {for props.children.iter()}
        </ContextProvider<LiveStreamService>>
    }
}
