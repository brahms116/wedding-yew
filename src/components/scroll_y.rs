use yew::{function_component, html, use_node_ref};

#[function_component(ScrollY)]
pub fn scroll_y() -> Html {
    let _anchor_ref = use_node_ref();
    html! {
        <div></div>
    }
}
