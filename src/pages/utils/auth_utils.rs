use tracing::{event, span, Level};

use super::*;

type A = AsyncResourceHandle<InviteInfo, ApiError>;

fn base_nav_items(id: &Option<String>) -> Vec<(NavDestination<Route, UrlQuery>, String)> {
    vec![
        (
            NavDestination::AppWithQuery(Route::Landing, UrlQuery { id: id.clone() }),
            "Home".into(),
        ),
        (
            NavDestination::AppWithQuery(Route::Story, UrlQuery { id: id.clone() }),
            "Our Story".into(),
        ),
        (
            NavDestination::External("https://github.com/brahms116/wedding-yew".into()),
            "Github".into(),
        ),
    ]
}

pub fn get_nav_items(
    status: &WeddingDayStatus,
    livestream_link: &str,
    id: &Option<String>,
) -> (
    Vec<(NavDestination<Route, UrlQuery>, String)>,
    NavDestination<Route, UrlQuery>,
) {
    let mut items = base_nav_items(id);
    let default = NavDestination::AppWithQuery(Route::Landing, UrlQuery { id: id.clone() });
    match status {
        WeddingDayStatus::Coming => {
            if let Some(id) = id {
                items.push((
                    NavDestination::AppWithQuery(
                        Route::FAQ,
                        UrlQuery {
                            id: Some(id.clone()),
                        },
                    ),
                    "FAQ".into(),
                ));
                items.push((
                    NavDestination::AppWithQuery(
                        Route::Schedule,
                        UrlQuery {
                            id: Some(id.clone()),
                        },
                    ),
                    "Schedule".into(),
                ));
                items.push((
                    NavDestination::AppWithQuery(
                        Route::RSVP,
                        UrlQuery {
                            id: Some(id.clone()),
                        },
                    ),
                    "RSVP".into(),
                ));
            }
        }
        WeddingDayStatus::Today => {
            if let Some(id) = id {
                items.push((
                    NavDestination::AppWithQuery(
                        Route::FAQ,
                        UrlQuery {
                            id: Some(id.clone()),
                        },
                    ),
                    "FAQ".into(),
                ));
                items.push((
                    NavDestination::AppWithQuery(
                        Route::Schedule,
                        UrlQuery {
                            id: Some(id.clone()),
                        },
                    ),
                    "Schedule".into(),
                ));
            }
            items.push((
                NavDestination::External(livestream_link.into()),
                "Livestream".into(),
            ));
        }
        WeddingDayStatus::Passed => {}
    }
    (items, default)
}

#[hook]
pub fn use_auth() -> (
    Vec<(NavDestination<Route, UrlQuery>, String)>,
    NavDestination<Route, UrlQuery>,
) {
    let invitation_service =
        use_context::<InvitationCtxValue>().expect("Try adding a provider for invitation service");
    let wedding_service =
        use_context::<WeddingDayCtxValue>().expect("Try providing a wedding service");
    let livestream_service =
        use_context::<LiveStreamService>().expect("Try providing a livestream service");
    let navigator = use_navigator().expect("Try placing this hook inside a router");
    let current_route = use_route::<Route>().expect("Try using this on a valid app route");
    let items = use_state(|| {
        get_nav_items(
            &wedding_service.relative_day_status,
            &livestream_service.0,
            &None,
        )
    });
    let id = use_query_id();
    {
        let dep = invitation_service.fetch_invite_handle().clone();
        let invitation_service = invitation_service.clone();
        let id = id.clone();
        let items = items.clone();
        use_effect_with_deps(
            move |_| {
                let span =
                    span!(target:"use-auth-hook", Level::DEBUG, "use-auth-on-invite-handle-change");
                let _enter = span.enter();
                event!(Level::INFO, "invite-handle changed");
                event!(
                    Level::DEBUG,
                    invite_handle =? invitation_service.fetch_invite_handle()
                );
                match invitation_service.fetch_invite_handle() {
                    A::None => {
                        if let Some(id) = id {
                            event!(Level::INFO, "call fetch invite-handle ");
                            invitation_service.fetch_invite(&id);
                        } else {
                            event!(Level::INFO, "perform auth route check");
                            match current_route {
                                Route::RSVP | Route::FAQ | Route::RSVPResult => {
                                    event!(Level::INFO, "reroute");
                                    navigator.push(&Route::Landing)
                                }
                                _ => {}
                            }
                        }
                    }
                    A::InitialLoad | A::SubsequentLoad(..) => {}
                    A::InitialErr(e) | A::SubsequentErr(e, ..) => {
                        event!(Level::ERROR, ?e);
                        event!(Level::INFO, "perform auth route check");
                        match current_route {
                            Route::RSVP | Route::FAQ | Route::RSVPResult => {
                                event!(Level::INFO, "reroute");
                                navigator.push(&Route::Landing)
                            }
                            _ => {}
                        }
                    }
                    A::Success(d) => {
                        event!(Level::INFO, "invite-handle successfully fetched");
                        if let Some(invite) = &d.invite {
                            let id = Some(invite.primary_invitee.id.clone());
                            items.set(get_nav_items(
                                &wedding_service.relative_day_status,
                                &livestream_service.0,
                                &id,
                            ))
                        } else {
                            match current_route {
                                Route::RSVP | Route::FAQ | Route::RSVPResult => {
                                    navigator.push(&Route::Landing)
                                }
                                _ => {}
                            }
                        }
                    }
                };
            },
            vec![dep],
        )
    }
    (*items).clone()
}
