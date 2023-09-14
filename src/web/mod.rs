mod error;
mod prelude;
mod api;

use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;

use prelude::Result;

pub fn routes() -> Router {
    Router::new()
        .nest("/api", api::routes())
        .route("/:hash", get(redirect))
}

async fn redirect(Path(hash): Path<String>) -> impl IntoResponse {
    // TODO: retrieve url from DB by "hash" and redirect to it.

    Redirect::permanent("https://google.com")
}