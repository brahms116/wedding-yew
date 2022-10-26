use crate::components::*;
use yew::{function_component, html, Html};

#[derive(PartialEq, Clone)]
struct OpacityController();

impl OpacityControl for OpacityController {
    fn get_opacity(&self, screen_height: f64, relative_y: f64) -> f64 {
        if screen_height == 0.0 {
            return 0.0;
        }
        let mut percentage = 2.0 * relative_y / screen_height;
        if percentage > 1.0 {
            percentage = 1.0;
        }

        percentage
    }
}

#[function_component(Splash)]
fn splash() -> Html {
    let msg: &str = "Hello, some message should be here, lets make it a super super long message so that wrapping occurs. Did you know that mia can get really cranky sometimes?";
    let words = msg.split(" ").collect::<Vec<&str>>();

    html! {
        <div class="w-screen h-screen flex justify-center items-center max-w-full">
            <ScrollOpacity<OpacityController> opacity_control={OpacityController()} class="flex flex-wrap w-2/5 justify-center">
                    {
                       words.into_iter().enumerate().map( |(i,w)|{html!{
                           <WordContainer delay={i as u32 * 100} word={w}/>
                       }}).collect::<Html>()
                    }
            </ScrollOpacity<OpacityController>>
        </div>
    }
}

#[function_component(Title)]
fn title() -> Html {
    html! {
        <div class="w-screen h-screen max-w-full">
        </div>
    }
}

#[derive(PartialEq, Clone)]
struct NavOpacityController();

impl OpacityControl for NavOpacityController {
    fn get_opacity(&self, screen_height: f64, relative_y: f64) -> f64 {
        if screen_height == 0.0 {
            return 0.0;
        }
        let mut percentage = 1.0 - relative_y / screen_height * 2.0;
        if percentage > 1.0 {
            percentage = 1.0;
        }

        percentage
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div>
            <ScrollOpacity<NavOpacityController>
                opacity_control={NavOpacityController()}
                class={"absolute top-[100vh]"}
            >
                <NavMenu/>
            </ScrollOpacity<NavOpacityController>>
            <Splash/>
            <Title/>
        </div>
    }
}
