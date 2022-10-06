use crate::components::*;
use yew::{function_component, html, Html};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
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
