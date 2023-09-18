use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use url::Url;
use crate::web::api::prelude::*;

pub(super) async fn create_shortened_link(
    State(db): State<sqlx::PgPool>,
    Json(payload): Json<CreateShortenedLinkPayload>,
) -> Result<Json<Value>> {
    let url = Url::parse(&payload.url).map_err(Error::from)?;

    let body = Json(json!({
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateShortenedLinkPayload {
    url: String,
}