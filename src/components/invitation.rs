use serde::{Deserialize, Serialize};
use tracing::debug;
use wasm_bindgen_futures::spawn_local;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invitee {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub rsvp: Option<bool>,
    pub dietary_requirements: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub primary_invitee: Invitee,
    pub dependents: Vec<Invitee>,
}

pub trait ApiResource<T, E, P>
where
    T: 'static + Send + Sync,
    E: std::error::Error,
    P: 'static + Send + Sync,
{
    fn data(&self) -> Option<T>;
    fn set_data(&self, setter: Box<dyn FnOnce(Option<T>) -> Option<T>>);
    fn fetch(&self, params: P);
    fn error(&self) -> Option<E>;
    fn loading(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub struct InviteInfo {
    pub invite: Option<Invitation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InviteProvidedInfo {
    pub invite: Option<InviteInfo>,
    pub set_invite: Callback<Option<InviteInfo>>,
    pub fetch_func: Callback<String>,
    pub error: Option<ApiError>,
    pub is_loading: bool,
}

impl ApiResource<InviteInfo, ApiError, String> for InviteProvidedInfo {
    fn data(&self) -> Option<InviteInfo> {
        self.invite.clone()
    }

    fn set_data(&self, setter: Box<dyn FnOnce(Option<InviteInfo>) -> Option<InviteInfo>>) {
        self.set_invite.emit(setter(self.data()))
    }

    fn fetch(&self, params: String) {
        self.fetch_func.emit(params)
    }

    fn error(&self) -> Option<ApiError> {
        self.error.clone()
    }

    fn loading(&self) -> bool {
        self.is_loading.clone()
    }
}

#[derive(PartialEq, Properties, Debug)]
pub struct InviteProviderProps<T>
where
    T: FetchInvite + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    pub children: Children,
    pub fetch_service: T,
}

#[function_component(InviteProvider)]
pub fn invite_provider<T>(props: &InviteProviderProps<T>) -> Html
where
    T: FetchInvite + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    debug!(InviteProviderProps = ?props);

    let data_handle = use_state(|| None::<InviteInfo>);
    let error_handle = use_state(|| None::<ApiError>);
    let loading_handle = use_state(|| false);

    let fetch_data = {
        let data_handle = data_handle.clone();
        let error_handle = error_handle.clone();
        let fetch_service = props.fetch_service.clone();
        let loading_handle = loading_handle.clone();
        Callback::from(move |id: String| {
            let fetch_service = fetch_service.clone();
            let data_handle = data_handle.clone();
            let error_handle = error_handle.clone();
            let loading_handle = loading_handle.clone();
            spawn_local(async move {
                loading_handle.set(true);
                let response = fetch_service.fetch_invite(&id).await;
                if let Ok(invite) = response {
                    data_handle.set(Some(InviteInfo {
                        invite: Some(invite),
                    }))
                } else if let Err(ApiError::NotInvited(_)) = response {
                    data_handle.set(None);
                } else {
                    let err = response.expect_err("Should have matched all other possibilities");
                    error_handle.set(Some(err))
                }
                loading_handle.set(false);
            });
        })
    };

    let manual_set = {
        let data_handle = data_handle.clone();
        Callback::from(move |state: Option<InviteInfo>| {
            data_handle.set(state);
        })
    };

    let provided_info = InviteProvidedInfo {
        invite: (*data_handle).clone(),
        set_invite: manual_set,
        fetch_func: fetch_data,
        error: (*error_handle).clone(),
        is_loading: *loading_handle,
    };

    debug!(InviteProvidedInfo = ?provided_info);

    html! {
        <ContextProvider<InviteProvidedInfo> context={provided_info}>
            {for props.children.iter()}
        </ContextProvider<InviteProvidedInfo>>
    }
}

#[derive(Deserialize, Debug)]
pub struct UrlQuery {
    pub id: Option<String>,
}

pub fn use_invitation<T>(route: Option<T>) -> InviteProvidedInfo
where
    T: 'static + Routable,
{
    let location = use_location().expect("Should have location");
    let history = use_history().expect("Should have history");
    let query = location
        .query::<UrlQuery>()
        .expect("Url params should be deserializable");

    debug!(url_query = ?query);
    let info = use_context::<InviteProvidedInfo>().expect("Context should be provided");

    {
        let route = route.clone();
        if let None = query.id {
            if let Some(route) = route {
                history.push(route);
            }
        }
    }

    if let Some(info) = info.data() {
        if let None = info.invite {
            if let Some(route) = route {
                history.push(route);
            }
        }
    }

    {
        let info = info.clone();
        let id = query.id.clone();
        use_effect_with_deps(
            move |_| {
                if let None = info.data() {
                    if let Some(id) = id {
                        info.fetch(id)
                    }
                }
                || {}
            },
            (),
        )
    }

    info
}
