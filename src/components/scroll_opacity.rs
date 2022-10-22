use crate::components::*;
use web_sys::HtmlElement;
use yew::{function_component, html, use_effect_with_deps, use_node_ref, Children, Properties};

pub trait OpacityControl: Clone + PartialEq + 'static {
    fn get_opacity(&self, screen_height: f64, relative_y: f64) -> f64;
}

#[derive(Properties, PartialEq)]
pub struct ScrollOpacityProps<T: OpacityControl> {
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    pub opacity_control: T,
}

#[function_component(ScrollOpacity)]
pub fn scroll_opacity<T: OpacityControl>(props: &ScrollOpacityProps<T>) -> Html {
    let element_ref = use_node_ref();
    let element_change_ref = element_ref.clone();
    let element_opacity_ref = element_ref.clone();

    let hook_info = use_element_window_info(&element_ref);
    let relative_y = hook_info.relative_y;
    let height = hook_info.window_height;

    let opacity_control = props.opacity_control.clone();

    use_effect_with_deps(
        move |_| {
            let element = element_opacity_ref
                .cast::<HtmlElement>()
                .expect("Html element should be Html Element");
            let percentage = opacity_control.get_opacity(height, relative_y);
            element
                .style()
                .set_property("opacity", &format!("{}", percentage))
                .expect("Should be able to set opacity");

            || {}
        },
        (relative_y, height, element_change_ref),
    );

    html! {
        <div ref={element_ref} class={props.class.clone()}>
            {for props.children.iter()}
        </div>
    }
}
