use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::web::api::prelude::Result;

pub(super) async fn create_shortened_link(payload: Json<CreateShortenedLinkPayload>) -> Result<Json<Value>> {
    let body = Json(json!({ // TODO: replace dummy response
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateShortenedLinkPayload {
    url: String,
}