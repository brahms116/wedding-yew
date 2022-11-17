use super::*;

#[derive(PartialEq, Clone)]
struct SplashOpacityControl();

impl OpacityControl for SplashOpacityControl {
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
pub fn splash() -> Html {
    let msg: &str = "
        Dear Linden and Emma, together with our families, we \
        joyfully request your company at the celebration of our marriage...";
    let words = msg.split(" ").collect::<Vec<&str>>();

    html! {
        <div class="w-screen h-screen flex justify-center items-center max-w-full">
            <ScrollOpacity<SplashOpacityControl>
                opacity_control={SplashOpacityControl()}
                class="flex flex-wrap w-2/5 justify-center"
            >
                    {
                       words.into_iter().enumerate().map( |(i,w)|{html!{
                           <WordContainer delay={i as u32 * 100} word={w}/>
                       }}).collect::<Html>()
                    }
            </ScrollOpacity<SplashOpacityControl>>
        </div>
    }
}
