use super::*;
use web_sys::Element;

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

pub struct ElementWindowInfoHook {
    pub relative_y: f64,
    pub window_height: f64,
}

#[hook]
pub fn use_element_window_info(element: &NodeRef) -> ElementWindowInfoHook {
    let ctx = use_context::<WindowInfo>().expect("There should be scroll context");
    let relative_y = use_state(|| 0.0);
    let y_return = *relative_y;
    let height_return = ctx.clone().height;
    let y_on_change = relative_y.clone();
    let ref_effect_dep = element.clone();
    let ref_height_calc = element.clone();

    use_effect_with_deps(
        move |_| {
            let element = ref_height_calc
                .cast::<Element>()
                .expect("Html element should be element");
            let element_y = element.get_bounding_client_rect().y();
            y_on_change.set(element_y);
            || {}
        },
        (ctx.scroll_height, ctx.height, ref_effect_dep),
    );

    ElementWindowInfoHook {
        relative_y: y_return,
        window_height: height_return,
    }
}
