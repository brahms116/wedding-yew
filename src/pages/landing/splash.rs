use super::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SplashProps {
    pub on_splash_click: Callback<MouseEvent>,
    pub is_loading: bool,
}

#[function_component(Splash)]
pub fn splash(props: &SplashProps) -> Html {
    let fade_out_class = use_state(|| "");
    let msg: &str = "\
        \"We love because he first loved us. ~1 John 4:19~\"\
    ";
    let words = msg.split(" ").collect::<Vec<&str>>();
    let label = if props.is_loading {
        "Please wait while we retrieve your invitation..."
    } else {
        "Enter"
    };

    let onclick = {
        let props = (*props).clone();
        let fade_out_class = fade_out_class.clone();
        Callback::from(move |e: MouseEvent| {
            if !props.is_loading {
                fade_out_class.set("animate-fade-splash");
                props.on_splash_click.emit(e);
            }
        })
    };

    let loading_class = if props.is_loading {
        "loading bg-bg text-black".to_string()
    } else {
        "bg-black text-white".to_string()
    };

    let pulse_class = if props.is_loading {
        "animate-pulse"
    } else {
        ""
    };
    html! {
        <div class={format!("
            w-screen fixed h-screen bg-bg z-20 top-0 flex justify-center
            items-center flex-col p-8 max-w-full {}", (*fade_out_class).clone())}
        >
            <div class="flex flex-wrap max-w-[500px] mb-4 italic justify-center text-xl">
            {
               words.into_iter().enumerate().map( |(i,w)|{html!{
                   <WordContainer delay={i as u32 * 100} word={w}/>
               }}).collect::<Html>()
            }
            </div>
            <button type="button"
                onclick={onclick}
                id="accept-splash-button"
                class={format!("
                    py-2 px-8 text-white 
                    animate-fade-splash-button
                    rounded-full {}
                ", loading_class)}
            >
                <div class={format!("{}",pulse_class)}>
                    {label}
                </div>
            </button>
        </div>
    }
}
