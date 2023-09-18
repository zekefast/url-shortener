pub(crate) mod error;
mod prelude;
mod links;

use axum::Router;
use axum::routing::post;

pub(crate) fn routes() -> crate::web::prelude::Router {
    Router::new()
        .route("/links", post(links::create_shortened_link))
}