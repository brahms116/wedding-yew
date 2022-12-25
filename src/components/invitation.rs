use super::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

type A<T, E> = AsyncResourceHandle<T, E>;

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
    pub fn is_coming(&self) -> bool {
        if self.primary_invitee.rsvp.unwrap_or(false) {
            return true;
        }
        for i in &self.dependents {
            if i.rsvp.unwrap_or(false) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InviteInfo {
    pub invite: Option<Invitation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvitationCtxValue {
    pub fetch_invite_handle: A<InviteInfo, ApiError>,
    pub fetch_invite: Callback<String>,
    pub rsvp_handle: A<bool, ApiError>,
    pub rsvp: Callback<Invitation>,
    pub reset_rsvp_handle_cb: Callback<()>,
    pub reset_fetch_handle_cb: Callback<()>,
}

pub trait InvitationService {
    fn fetch_invite_handle(&self) -> &A<InviteInfo, ApiError>;
    fn fetch_invite(&self, id: &str);
    fn rsvp(&self, invite: &Invitation);
    fn rsvp_handle(&self) -> &A<bool, ApiError>;
    fn reset_rsvp_handle(&self);
}

impl InvitationService for InvitationCtxValue {
    fn fetch_invite_handle(&self) -> &A<InviteInfo, ApiError> {
        &self.fetch_invite_handle
    }

    fn fetch_invite(&self, id: &str) {
        self.fetch_invite.emit(id.to_string());
    }

    fn rsvp(&self, invite: &Invitation) {
        self.rsvp.emit((*invite).clone());
    }

    fn rsvp_handle(&self) -> &A<bool, ApiError> {
        &self.rsvp_handle
    }
    fn reset_rsvp_handle(&self) {
        self.reset_rsvp_handle_cb.emit(());
    }
}

#[derive(PartialEq, Properties, Debug)]
pub struct InviteProviderProps<T>
where
    T: InviteApi2 + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    pub children: Children,
    pub api_service: T,
}

#[function_component(InviteProvider)]
pub fn invite_provider<T>(props: &InviteProviderProps<T>) -> Html
where
    T: InviteApi2 + PartialEq + Clone + 'static + Send + Sync + std::fmt::Debug,
{
    let fetch_handle = use_state(|| A::<InviteInfo, ApiError>::None);
    let save_handle = use_state(|| A::<bool, ApiError>::None);

    let save_data = {
        let save_handle = save_handle.clone();
        let api_service = props.api_service.clone();
        Callback::from(move |invite: Invitation| {
            let save_handle = save_handle.clone();
            let api_service = api_service.clone();
            let invite = invite.clone();
            spawn_local(async move {
                match &*save_handle {
                    A::SubsequentLoad(..) | A::InitialLoad => return,
                    A::SubsequentErr(.., d) | A::Success(d) => {
                        save_handle.set(A::SubsequentLoad(d.clone()));
                    }
                    A::None | A::InitialErr(..) => {
                        save_handle.set(A::InitialLoad);
                    }
                };

                let url = api_service.get_url();
                let response = save_invite(url, &invite).await;
                if let Err(err) = response {
                    // TODO: Parse error properly and set it in save_handle
                } else {
                    save_handle.set(A::Success(true));
                }
            })
        })
    };

    let fetch_data = {
        let fetch_handle = fetch_handle.clone();
        let api_service = props.api_service.clone();
        Callback::from(move |id: String| {
            let fetch_handle = fetch_handle.clone();
            let api_service = api_service.clone();
            spawn_local(async move {
                if fetch_handle.loading() {
                    return;
                }
                if let Some(d) = fetch_handle.data() {
                    fetch_handle.set(A::SubsequentLoad(d.clone()))
                } else {
                    fetch_handle.set(A::InitialLoad)
                }

                let response = fetch_invite(&api_service.get_url(), &id).await;
                if let Ok(invite) = response {
                    fetch_handle.set(A::Success(InviteInfo {
                        invite: Some(invite),
                    }))
                } else if let Err(ApiError::NotInvited(_)) = response {
                    fetch_handle.set(A::Success(InviteInfo { invite: None }));
                } else {
                    let err = response.expect_err("Should have matched all other possibilities");
                    if let A::SubsequentLoad(d) = &*fetch_handle {
                        fetch_handle.set(A::SubsequentErr(err, d.clone()))
                    } else {
                        fetch_handle.set(A::InitialErr(err))
                    }
                }
            });
        })
    };

    let reset_save_request = {
        let save_handle = save_handle.clone();
        Callback::from(move |_: ()| {
            let mut new_save_handle = (*save_handle).clone();
            new_save_handle.reset();
            save_handle.set(new_save_handle);
        })
    };

    let reset_fetch_request = {
        let fetch_handle = fetch_handle.clone();
        Callback::from(move |_: ()| {
            let mut new_fetch_handle = (*fetch_handle).clone();
            new_fetch_handle.reset();
            fetch_handle.set(new_fetch_handle);
        })
    };

    let provided_info = InvitationCtxValue {
        fetch_invite_handle: (*fetch_handle).clone(),
        fetch_invite: fetch_data,
        rsvp_handle: (*save_handle).clone(),
        rsvp: save_data,
        reset_rsvp_handle_cb: reset_save_request,
        reset_fetch_handle_cb: reset_fetch_request,
    };

    html! {
        <ContextProvider<InvitationCtxValue> context={provided_info}>
            {for props.children.iter()}
        </ContextProvider<InvitationCtxValue>>
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UrlQuery {
    pub id: Option<String>,
}

#[hook]
pub fn use_query_id() -> Option<String> {
    let location = use_location().expect("Should have location");
    let query = location
        .query::<UrlQuery>()
        .expect("Url params should be deserializable");
    query.id
}
