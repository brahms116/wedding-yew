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
    ]
}

pub fn get_nav_items(
    status: &WeddingDayStatus,
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
            }
            items.push((
                NavDestination::AppWithQuery(Route::LiveStream, UrlQuery { id: id.clone() }),
                "Live stream".into(),
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
    let wedding_service = use_context::<WeddingDayInfo>().expect("Try providing a wedding service");
    let navigator = use_navigator().expect("Try placing this hook inside a router");
    let current_route = use_route::<Route>().expect("Try using this on a valid app route");
    let items = use_state(|| get_nav_items(&wedding_service.relative_day_status, &None));
    let id = use_query_id();
    {
        let dep = invitation_service.fetch_invite_handle().clone();
        let invitation_service = invitation_service.clone();
        let id = id.clone();
        let items = items.clone();
        use_effect_with_deps(
            move |_| {
                match invitation_service.fetch_invite_handle() {
                    A::None => {
                        if let Some(id) = id {
                            invitation_service.fetch_invite(&id);
                        } else {
                            match current_route {
                                Route::RSVP | Route::FAQ | Route::RSVPResult => {
                                    navigator.push(&Route::Landing)
                                }
                                _ => {}
                            }
                        }
                    }
                    A::InitialLoad | A::SubsequentLoad(..) => {}
                    A::InitialErr(..) | A::SubsequentErr(..) => match current_route {
                        Route::RSVP | Route::FAQ | Route::RSVPResult => {
                            navigator.push(&Route::Landing)
                        }
                        _ => {}
                    },
                    A::Success(d) => {
                        if let Some(invite) = &d.invite {
                            let id = Some(invite.primary_invitee.id.clone());
                            items.set(get_nav_items(&wedding_service.relative_day_status, &id))
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
