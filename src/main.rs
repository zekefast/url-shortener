#![allow(unused)] // TODO: remove later before release to production.

mod error;
mod prelude;
mod utils;
mod api;

use std::net::SocketAddr;
use axum::handler::HandlerWithoutStateExt;
use axum::Router;
use crate::prelude::*;

const DEFAULT_LISTEN_PORT: u16 = 8080;
const DEFAULT_LISTEN_HOST: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() -> Result<()> {
    let listen_address = SocketAddr::from((DEFAULT_LISTEN_HOST, DEFAULT_LISTEN_PORT));

    let routes_api = Router::new();

    let routes_all = Router::new()
        .nest("/api", routes_api);

    axum::Server::bind(&listen_address)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
