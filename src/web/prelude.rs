use crate::web::error::Error;

pub type Result<T> = core::result::Result<T, Error>;