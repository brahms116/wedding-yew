use gloo::timers::callback::Timeout;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Properties};

#[derive(Properties, PartialEq)]
pub struct WordContainerProps {
    pub word: String,
    pub delay: u32,
}

#[function_component(WordContainer)]
pub fn word_container(props: &WordContainerProps) -> Html {
    let class = use_state(|| "opacity-0".to_owned());

    let cb = {
        let class = class.clone();
        move || class.set("animate-fade".into())
    };

    let delay = props.clone().delay;

    use_effect_with_deps(
        move |_| {
            Timeout::new(delay, cb).forget();
            || {}
        },
        (),
    );

    html! {
        <div class={classes!(vec![(*class).clone(),"mr-1 text-xl".into()])}>{props.word.clone()}</div>
    }
}
