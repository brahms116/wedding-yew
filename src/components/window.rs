use yew::{
    function_component, html, use_effect_with_deps, use_state, Children, ContextProvider,
    Properties,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct WindowInfo {
    pub height: f64,
    pub width: f64,
    pub scroll_height: f64,
}

#[derive(Properties, PartialEq)]
pub struct WindowProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(WindowProvider)]
pub fn window_provider(props: &WindowProviderProps) -> Html {
    let state = use_state(|| WindowInfo {
        width: 0.0,
        height: 0.0,
        scroll_height: 0.0,
    });

    let state_init_handle = state.clone();

    let update = move || {
        let doc = gloo_utils::document()
            .document_element()
            .expect("Should have document");
        let height = doc.client_height() as f64;
        let width = doc.client_width() as f64;

        let scroll_height = gloo_utils::window()
            .scroll_y()
            .expect("Document should have scroll y");

        state_init_handle.set(WindowInfo {
            height,
            width,
            scroll_height,
        });
    };

    let resize_update = update.clone();
    let scroll_update = update.clone();

    let _resize_handle = use_state(|| {
        gloo_events::EventListener::new(&gloo_utils::window(), "resize", move |_| resize_update())
    });

    let _scroll_handle = use_state(|| {
        gloo_events::EventListener::new(&gloo_utils::window(), "scroll", move |_| scroll_update())
    });

    use_effect_with_deps(
        move |_| {
            update();
            || {}
        },
        (),
    );

    html! {
        <ContextProvider<WindowInfo> context={(*state).clone()}>
            {for props.children.iter()}
        </ContextProvider<WindowInfo>>
    }
}
