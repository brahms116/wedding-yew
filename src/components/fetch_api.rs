use super::*;
use async_trait::async_trait;
use thiserror::Error;

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

#[async_trait]
impl InviteApi for FetchService {
    async fn fetch_invite(&self, id: &str) -> Result<Invitation, ApiError> {
        Ok(Invitation {
            primary_invitee: Invitee {
                id: "myid".into(),
                fname: "David".into(),
                lname: "Kwong".into(),
                rsvp: Some(true),
                dietary_requirements: "".into(),
            },
            dependents: vec![
                Invitee {
                    id: "myid2".into(),
                    fname: "Mia".into(),
                    lname: "Huang".into(),
                    rsvp: None,
                    dietary_requirements: "My water needs to be warm".into(),
                },
                Invitee {
                    id: "myid3".into(),
                    fname: "William".into(),
                    lname: "Kwong".into(),
                    rsvp: Some(false),
                    dietary_requirements: "Big portions".into(),
                },
            ],
        })
        // Err(ApiError::NotInvited("myid5".into()))
        // Err(ApiError::FetchFailure("Connection failed".into()))
    }

    async fn save_invite(&self, invitation: &Invitation) -> Result<bool, ApiError> {
        Ok(true)
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ApiError {
    #[error("Invitee {0} is not invited, sorry!")]
    NotInvited(String),
    #[error("Failed to fetch invite, reason: {0}")]
    FetchFailure(String),
}

#[async_trait]
pub trait InviteApi {
    async fn fetch_invite(&self, id: &str) -> Result<Invitation, ApiError>;

    async fn save_invite(&self, invitation: &Invitation) -> Result<bool, ApiError>;
}
