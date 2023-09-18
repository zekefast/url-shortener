use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use url::Url;
use crate::service::link;
use crate::web::api::prelude::*;

pub(super) async fn create_shortened_link(
    State(db): State<sqlx::PgPool>,
    Json(payload): Json<CreateShortenedLinkPayload>,
) -> Result<Json<Value>> {
    let url = Url::parse(&payload.url).map_err(Error::from)?;

    let shortened_url = link::shorten(url, db);

    let body = Json(json!({
        "url": shortened_url
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateShortenedLinkPayload {
    url: String,
}