use async_trait::async_trait;
use thiserror::Error;

use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct FetchService(pub String);

#[async_trait]
impl FetchInvite for FetchService {
    async fn fetch_invite(&self, id: &str) -> Result<Invitation, ApiError> {
        // Ok(Invitation {
        //     primary_invitee: Invitee {
        //         id: "myid".into(),
        //         fname: "David".into(),
        //         lname: "Kwong".into(),
        //         rsvp: None,
        //         dietary_requirements: "".into(),
        //     },
        //     dependents: vec![
        //         Invitee {
        //             id: "myid2".into(),
        //             fname: "Mia".into(),
        //             lname: "Huang".into(),
        //             rsvp: None,
        //             dietary_requirements: "".into(),
        //         },
        //         Invitee {
        //             id: "myid3".into(),
        //             fname: "William".into(),
        //             lname: "Kwong".into(),
        //             rsvp: None,
        //             dietary_requirements: "".into(),
        //         },
        //     ],
        // })
        // Err(ApiError::NotInvited("myid5".into()))
        Err(ApiError::FetchFailure("Connecttion failed".into()))
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
pub trait FetchInvite {
    async fn fetch_invite(&self, id: &str) -> Result<Invitation, ApiError>;
}
