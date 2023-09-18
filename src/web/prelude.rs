use crate::ApplicationState;
use crate::web::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type Router = axum::Router<ApplicationState>;