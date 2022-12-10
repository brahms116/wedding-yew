use super::*;
use serde::{Deserialize, Serialize};
use tracing::debug;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invitee {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub rsvp: Option<bool>,
    pub dietary_requirements: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub primary_invitee: Invitee,
    pub dependents: Vec<Invitee>,
}

impl Invitation {
    pub fn get_fnames(&self) -> Vec<String> {
        let mut result = vec![self.primary_invitee.fname.clone()];
        for i in &self.dependents {
            result.push(i.fname.clone());
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InviteInfo {
    pub invite: Option<Invitation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvitationCtxValue {
    pub fetch_invite_handle: AsyncResourceHandle<InviteInfo, ApiError>,
    pub fetch_invite: Callback<String>,
}

pub trait InvitationService {
    fn invite_data(&self) -> &AsyncResourceHandle<InviteInfo, ApiError>;
    fn fetch_invite(&self, id: &str);
}

impl InvitationService for InvitationCtxValue {
    fn invite_data(&self) -> &AsyncResourceHandle<InviteInfo, ApiError> {
        &self.fetch_invite_handle
    }

    fn fetch_invite(&self, id: &str) {
        self.fetch_invite.emit(id.to_string());
    }
}

#[derive(PartialEq, Properties, Debug)]
pub struct InviteProviderProps<T>
where
    T: InviteApi + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    pub children: Children,
    pub api_service: T,
}

#[function_component(InviteProvider)]
pub fn invite_provider<T>(props: &InviteProviderProps<T>) -> Html
where
    T: InviteApi + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    debug!(InviteProviderProps = ?props);

    let fetch_handle = use_state(|| AsyncResourceHandle::<InviteInfo, ApiError>::None);

    let fetch_data = {
        let fetch_handle = fetch_handle.clone();
        let api_service = props.api_service.clone();
        Callback::from(move |id: String| {
            let fetch_handle = fetch_handle.clone();
            let api_service = api_service.clone();
            spawn_local(async move {
                match &*fetch_handle {
                    AsyncResourceHandle::None | AsyncResourceHandle::InitialErr(..) => {
                        fetch_handle.set(AsyncResourceHandle::InitialLoad)
                    }
                    AsyncResourceHandle::Success(d) | AsyncResourceHandle::SubsequentErr(.., d) => {
                        fetch_handle.set(AsyncResourceHandle::SubsequentLoad(d.clone()))
                    }
                    AsyncResourceHandle::SubsequentLoad(..) | AsyncResourceHandle::InitialLoad => {
                        return
                    }
                };

                let response = api_service.fetch_invite(&id).await;
                if let Ok(invite) = response {
                    fetch_handle.set(AsyncResourceHandle::Success(InviteInfo {
                        invite: Some(invite),
                    }))
                } else if let Err(ApiError::NotInvited(_)) = response {
                    fetch_handle.set(AsyncResourceHandle::Success(InviteInfo { invite: None }));
                } else {
                    let err = response.expect_err("Should have matched all other possibilities");
                    if let AsyncResourceHandle::SubsequentLoad(d) = &*fetch_handle {
                        fetch_handle.set(AsyncResourceHandle::SubsequentErr(err, d.clone()))
                    } else {
                        fetch_handle.set(AsyncResourceHandle::InitialErr(err))
                    }
                }
            });
        })
    };

    let provided_info = InvitationCtxValue {
        fetch_invite: fetch_data,
        fetch_invite_handle: (*fetch_handle).clone(),
    };

    debug!(InviteProvidedInfo = ?provided_info);

    html! {
        <ContextProvider<InvitationCtxValue> context={provided_info}>
            {for props.children.iter()}
        </ContextProvider<InvitationCtxValue>>
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UrlQuery {
    pub id: Option<String>,
}

#[hook]
pub fn use_query_id() -> Option<String> {
    let location = use_location().expect("Should have location");
    let query = location
        .query::<UrlQuery>()
        .expect("Url params should be deserializable");
    debug!(url_query = ?query);
    query.id
}
