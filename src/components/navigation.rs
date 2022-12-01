use gloo_utils::document;
use std::hash::Hash;
use web_sys::HtmlElement;

use super::*;

/// A destination to route to
///
/// # Generics
/// T - Route type of the application
#[derive(PartialEq, Debug, Clone)]
pub enum NavDestination<T>
where
    T: Routable,
{
    /// An in-app route
    App(T),

    /// An external url
    External(String),
}

impl<T> Default for NavDestination<T>
where
    T: Routable,
{
    fn default() -> Self {
        Self::External(String::from("/"))
    }
}

/// Props for [NavLink]
///
/// # Generics
/// * T - Is the route type of the application
#[derive(Properties, PartialEq)]
pub struct NavLinkProps<T>
where
    T: Routable + 'static,
{
    /// The route the link links to
    route: T,
    /// The label of the link
    label: String,
    /// Whether to apply mobile styling
    is_mobile: bool,
}

/// Component for navigation items in the nav menu
///
/// # Props
/// * [NavLinkProps]
///
/// # Generics
/// * T  - Application route type
#[function_component(NavLink)]
pub fn nav_link<T>(props: &NavLinkProps<T>) -> Html
where
    T: Routable + 'static,
{
    let class = if props.is_mobile { "mb-8" } else { "ml-8" };
    html! {
        <div class={format!("{} hover:text-slate-400",class)}>
            <Link<T> to={props.route.clone()}>
                {props.label.clone()}
            </Link<T>>
        </div>
    }
}

/// Props for [NavMenu]
///
/// # Generics
/// * T - Route type of the application
#[derive(Properties, PartialEq)]
pub struct NavMenuProps<T>
where
    T: Eq + 'static + Hash + Routable + Default,
{
    pub routes: Vec<(T, String)>,
}

/// Navigation menu for the application
///
/// # Props
/// * [NavMenuProps]
///
/// # Generics
/// * T - Route type of the application
#[function_component(NavMenu)]
pub fn nav_menu<T>(props: &NavMenuProps<T>) -> Html
where
    T: Eq + 'static + Hash + Routable + Default,
{
    let menu_open = use_state(|| false);

    let current_route = use_route::<T>();

    let is_same_route = |route: &T| -> bool {
        if let Some(current_route) = current_route {
            return *route == current_route;
        }
        false
    };

    let desktop_links_css = "
        md:flex text-sm text-bold hidden
    ";

    let menu_icon_css = "
        md:hidden hover:text-slate-400 cursor-pointer
    ";

    let mobile_menu_css = "
        md:hidden fixed top-0 left-0 bg-bg min-h-screen w-full
        flex flex-col justify-center items-center text-sm text-bold
        transition-transform z-10
    ";

    let menu_style = if *menu_open {
        "transform: translateX(0%)"
    } else {
        "transform: translateX(100%)"
    };

    let cb = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| {
            menu_open.set(!(*menu_open));
        })
    };

    {
        let menu_open_dep = *menu_open.clone();
        let menu_open_predicate = *menu_open.clone();
        use_effect_with_deps(
            move |_| {
                let body: HtmlElement = document().body().unwrap();
                if menu_open_predicate {
                    body.style().set_property("overflow-y", "hidden").unwrap();
                } else {
                    body.style().set_property("overflow-y", "auto").unwrap();
                }

                || {
                    document()
                        .body()
                        .unwrap()
                        .style()
                        .set_property("overflow-y", "auto")
                        .unwrap();
                }
            },
            menu_open_dep,
        );
    }

    let menu_icon = if *menu_open { "fa-close" } else { "fa-bars" };

    html! {
        <>
            <div class={"
                    fixed top-0 w-full left-0 p-8 flex justify-between h-16 items-center
                    bg-bg z-20
                "}
            >
                <div class={"text-xl cursor-pointer"}>
                    {"Mia & David"}
                </div>
                <div class={desktop_links_css}>
                    {
                        props.routes
                            .clone()
                            .into_iter()
                            .filter(|a|!is_same_route.clone()(&a.0))
                            .map(|(a,b)|{
                                html!{
                                    <NavLink<T> route={a} label={b} is_mobile={false}/>
                                }
                            }).collect::<Html>()
                    }
                </div>
                <div class={menu_icon_css} onclick={cb}>
                    <i class={format!("fa-solid {}", menu_icon)}/>
                </div>
            </div>
            <div class={mobile_menu_css} style={menu_style}>
                {
                    props.routes
                        .clone()
                        .into_iter()
                        .filter(|a|!is_same_route.clone()(&a.0))
                        .map(|(a,b)|{
                            html!{
                                <NavLink<T> route={a} label={b} is_mobile={true}/>
                            }
                        }).collect::<Html>()
                }
            </div>
        </>
    }
}
