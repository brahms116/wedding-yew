use gloo::timers::callback::Timeout;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Html, Properties};

mod pages;

#[derive(Properties, PartialEq)]
struct WordContainerProps {
    pub word: String,
    pub delay: u32,
}

#[function_component(WordContainer)]
fn word_container(props: &WordContainerProps) -> Html {
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
        <div class={classes!(vec![(*class).clone(),"mr-1 text-2xl".into()])}>{props.word.clone()}</div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let msg: &str = "Hello, some message should be here, lets make it a super super long message so that wrapping occurs. Did you know that mia can get really cranky sometimes?";
    let words = msg.split(" ").collect::<Vec<&str>>();

    html! {
        <div class="w-screen h-screen flex justify-center items-center">
            <div class="flex flex-wrap w-2/5 justify-center">
                {
                   words.into_iter().enumerate().map( |(i,w)|{html!{
                       <WordContainer delay={i as u32 * 100} word={w}/>
                   }}).collect::<Html>()
                }
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
