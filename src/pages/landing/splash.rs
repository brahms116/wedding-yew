use super::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SplashProps {
    pub on_splash_click: Callback<MouseEvent>,
    pub is_loading: bool,
}

#[function_component(Splash)]
pub fn splash(props: &SplashProps) -> Html {
    let msg: &str = "
        \"We love because he first loved us. ~1 John 4:19~\"
    ";
    let words = msg.split(" ").collect::<Vec<&str>>();

    let label = if props.is_loading {
        "Loading..."
    } else {
        "Amen"
    };

    let onclick = {
        let props = (*props).clone();
        Callback::from(move |e: MouseEvent| {
            if !props.is_loading {
                props.on_splash_click.emit(e);
            }
        })
    };

    html! {
        <div class="
            w-screen fixed h-screen bg-bg z-20 top-0 flex justify-center
            items-center flex-col p-8 max-w-full
        ">
            <div class="flex flex-wrap max-w-[500px] mb-4 italic justify-center text-xl">
            {
               words.into_iter().enumerate().map( |(i,w)|{html!{
                   <WordContainer delay={i as u32 * 100} word={w}/>
               }}).collect::<Html>()
            }
            </div>
            <button type="button"
                onclick={onclick}
                class="
                    py-2 px-8 bg-black text-white 
                    animate-fade-slow
                    rounded-full
                "
            >{label}</button>
        </div>
    }
}
