use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::web::api;

#[derive(thiserror::Error, Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
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