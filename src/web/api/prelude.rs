use crate::web::api::error::Error;

pub type Result<T> = core::result::Result<T, Error>;