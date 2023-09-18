use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Clone/*, Serialize*/)]
//#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),
}

impl Error {
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response
            .extensions_mut()
            .insert(self);

        response
    }
}