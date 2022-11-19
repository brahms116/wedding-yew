use super::*;

#[derive(Properties, PartialEq)]
pub struct SplashProps {
    pub on_splash_click: Callback<MouseEvent>,
}

#[function_component(Splash)]
pub fn splash(props: &SplashProps) -> Html {
    let msg: &str = "
        \"We love because he first loved us. ~1 John 4:19~\"
    ";
    let words = msg.split(" ").collect::<Vec<&str>>();

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
                onclick={props.on_splash_click.clone()}
                class="
                    py-2 px-8 bg-black text-white 
                    rounded-full
                "
            >{"Amen"}</button>
        </div>
    }
}
