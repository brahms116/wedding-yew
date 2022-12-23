use super::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: Option<Invitation>,
    pub err: Option<ErrResponse>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrResponse {
    pub err_type: String,
    pub msg: Option<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum AsyncResourceHandle<T, E>
where
    T: 'static + Send + Sync,
    E: std::error::Error,
{
    None,
    InitialLoad,
    Success(T),
    SubsequentLoad(T),
    InitialErr(E),
    SubsequentErr(E, T),
}

impl<T, E> AsyncResourceHandle<T, E>
where
    T: 'static + Send + Sync,
    E: std::error::Error,
{
    pub fn loading(&self) -> bool {
        match self {
            Self::InitialLoad => true,
            Self::SubsequentLoad(_) => true,
            _ => false,
        }
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Success(d) | Self::SubsequentLoad(d) | Self::SubsequentErr(.., d) => {
                return Some(d)
            }
            _ => None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::None
    }
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

pub trait PersistResource<T, E>
where
    T: 'static + Send + Sync,
    E: std::error::Error,
{
    fn save(&self, data: T);
    fn error(&self) -> Option<E>;
    fn loading(&self) -> bool;
}

#[derive(Clone, PartialEq, Debug)]
pub struct FetchService(pub String);

pub async fn fetch_invite(url: &str, id: &str) -> Result<Invitation, ApiError> {
    let request = Request::post(url)
        .json(&serde_json::json!({
            "function":"fetchInvitation",
            "params": {
                "id":id
            }
        }))
        .expect("Should serialize correctly")
        .send()
        .await;

    if let Err(err) = request {
        return Err(ApiError::FetchFailure(err.to_string()));
    }

    let response = request.expect("Should handle err");
    let json_response = response
        .json::<ApiResponse>()
        .await
        .map_err(|e| ApiError::FetchFailure(e.to_string()))?;

    if let None = json_response.data {
        return Err(ApiError::FetchFailure("Some error".to_string()));
    }
    Ok(json_response.data.unwrap())
}

pub async fn save_invite(url: &str, invitation: &Invitation) -> Result<Invitation, ApiError> {
    let request = Request::post(url)
        .json(&serde_json::json!({
            "function":"updateInvitation",
            "params": {
                "invitation":serde_json::json!(invitation)
            }
        }))
        .expect("Should serialize correctly")
        .send()
        .await;

    if let Err(err) = request {
        return Err(ApiError::FetchFailure(err.to_string()));
    }

    let response = request.expect("Should handle err");
    let json_response = response
        .json::<ApiResponse>()
        .await
        .map_err(|e| ApiError::FetchFailure(e.to_string()))?;
    if let None = json_response.data {
        return Err(ApiError::FetchFailure("Some error".to_string()));
    }
    Ok(json_response.data.unwrap())
}

pub trait InviteApi2 {
    fn get_url(&self) -> &str;
}

impl InviteApi2 for FetchService {
    fn get_url(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ApiError {
    #[error("Invitee {0} is not invited, sorry!")]
    NotInvited(String),
    #[error("Failed to fetch invite, reason: {0}")]
    FetchFailure(String),
}
