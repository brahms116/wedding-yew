use crate::components::*;
use web_sys::{Element, HtmlElement};
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_node_ref, use_state, Children,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct ScrollOpacityProps {
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(ScrollOpacity)]
pub fn scroll_opacity(props: &ScrollOpacityProps) -> Html {
    let ctx = use_context::<WindowInfo>().expect("There should be scroll context");
    let relative_y = use_state(|| 0.0);
    let on_change_relative_y = relative_y.clone();
    let on_change_y = relative_y.clone();
    let element_ref = use_node_ref();
    let on_change_ref = element_ref.clone();
    let calc_height_ref = element_ref.clone();
    let opacity_ref = element_ref.clone();
    let opacity_ref_change = element_ref.clone();

    use_effect_with_deps(
        move |_| {
            let element = calc_height_ref
                .cast::<Element>()
                .expect("Html element should be element");
            let element_y = element.get_bounding_client_rect().y();
            on_change_y.set(element_y);
            || {}
        },
        (ctx.scroll_height, ctx.height, on_change_ref),
    );

    use_effect_with_deps(
        move |_| {
            let element = opacity_ref
                .cast::<HtmlElement>()
                .expect("Html element should be Html Element");
            if ctx.height != 0.0 {
                let mut percentage = 2.0 * *relative_y / ctx.height;
                if percentage > 1.0 {
                    percentage = 1.0
                }
                element
                    .style()
                    .set_property("opacity", &format!("{}", percentage))
                    .expect("Should be able to set opacity");
            }

            || {}
        },
        (on_change_relative_y, ctx.height, opacity_ref_change),
    );

    html! {
        <div ref={element_ref} class={props.class.clone()}>
            {for props.children.iter()}
        </div>
    }
}
