//! Module containing all the pages of the app
//!
//! The only exports are the enum [Route] and the [switch] function which are used to start the yew app

use super::components::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod landing;
mod route;
mod rsvp;

pub use route::*;
