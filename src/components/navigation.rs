use gloo_utils::document;
use serde::Serialize;
use std::hash::Hash;
use web_sys::HtmlElement;

use super::*;

/// A destination to route to
///
/// # Generics
/// T - Route type of the application
/// Q - The query parameters available
#[derive(PartialEq, Debug, Clone)]
pub enum NavDestination<T, Q>
where
    T: Routable,
    Q: Serialize,
{
    /// An in-app route
    App(T),

    /// An in-app route with query parameters
    AppWithQuery(T, Q),

    /// An external url
    External(String),
}

impl<T, Q> Default for NavDestination<T, Q>
where
    T: Routable,
    Q: Serialize,
{
    fn default() -> Self {
        Self::External(String::from("/"))
    }
}

/// Props for [NavLink]
///
/// # Generics
/// * T - Is the route type of the application
/// * Q - Is the available queries for the route
#[derive(Properties, PartialEq)]
pub struct NavLinkProps<T, Q>
where
    T: Routable + 'static,
    Q: Serialize + PartialEq + 'static + Clone,
{
    /// The route the link links to
    route: NavDestination<T, Q>,
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
pub fn nav_link<T, Q>(props: &NavLinkProps<T, Q>) -> Html
where
    T: Routable + 'static,
    Q: Serialize + PartialEq + 'static + Clone,
{
    let class = if props.is_mobile { "mb-8" } else { "ml-8" };
    let navigator = use_navigator().expect("Navigator is missing");

    if let NavDestination::App(route) = props.route.clone() {
        html! {
            <div class={format!("{} cursor-pointer hover:text-slate-400",class)}>
                <Link<T> to={route}>
                    {props.label.clone()}
                </Link<T>>
            </div>
        }
    } else if let NavDestination::External(link) = props.route.clone() {
        html! {
            <div class={format!("{} cursor-pointer hover:text-slate-400",class)}>
                    <a href={link}>{props.label.clone()}</a>
            </div>
        }
    } else if let NavDestination::AppWithQuery(route, query) = props.route.clone() {
        html! {
            <div
                onclick={Callback::from(move |_| {
                    navigator.push_with_query(&route, &query).expect("Navigation failed");
                })}
                class={format!("{} cursor-pointer hover:text-slate-400",class)}
            >
                {props.label.clone()}
            </div>
        }
    } else {
        html! {}
    }
}

/// Props for [NavMenu]
///
/// # Generics
/// * T - Route type of the application
#[derive(Properties, PartialEq)]
pub struct NavMenuProps<T, Q>
where
    T: Eq + 'static + Hash + Routable + Default,
    Q: 'static + PartialEq + Serialize,
{
    pub routes: Vec<(NavDestination<T, Q>, String)>,
}

/// Navigation menu for the application
///
/// # Props
/// * [NavMenuProps]
///
/// # Generics
/// * T - Route type of the application
#[function_component(NavMenu)]
pub fn nav_menu<T, Q>(props: &NavMenuProps<T, Q>) -> Html
where
    T: Eq + 'static + Hash + Routable + Default,
    Q: PartialEq + Serialize + 'static + Clone,
{
    let menu_open = use_state(|| false);

    let current_route = use_route::<T>();

    let is_same_route = |route: &NavDestination<T, Q>| -> bool {
        match route {
            NavDestination::App(route) | NavDestination::AppWithQuery(route, ..) => {
                if let Some(current_route) = current_route {
                    return *route == current_route;
                }
            }
            _ => (),
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
                <div class={"text-xl"}>
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
                                    <NavLink<T,Q> route={a} label={b} is_mobile={false}/>
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
                                <NavLink<T,Q> route={a} label={b} is_mobile={true}/>
                            }
                        }).collect::<Html>()
                }
            </div>
        </>
    }
}
