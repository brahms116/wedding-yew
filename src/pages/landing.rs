mod splash;
mod title;
use super::*;
use splash::*;
use title::*;

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
