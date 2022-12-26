use super::*;
use gloo::timers::callback::Timeout;

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

    {
        let delay = props.clone().delay;
        let word = props.word.clone();
        use_effect_with_deps(
            move |_| {
                Timeout::new(delay, cb).forget();
                || {}
            },
            word,
        );
    }

    html! {
        <div class={classes!(vec![(*class).clone(),"mr-1 text-xl".into()])}>{props.word.clone()}</div>
    }
}
