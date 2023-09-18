use crate::service::error::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;