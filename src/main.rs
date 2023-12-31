#![allow(unused)] // TODO: remove later before release to production.

mod error;
mod prelude;
mod utils;
mod web;
mod service;
mod model;
mod shortened_url;

use std::{
    net::SocketAddr,
    time::Duration,
};
use axum::extract::FromRef;
use axum::handler::HandlerWithoutStateExt;
use axum::Router;

use dotenvy::dotenv;
use sqlx::{
    postgres::PgPoolOptions,
    PgPool
};

use crate::prelude::*;


const DEFAULT_LISTEN_PORT: u16 = 8080;
const DEFAULT_LISTEN_HOST: [u8; 4] = [127, 0, 0, 1];



#[derive(Clone)]
pub struct ApplicationState {
    pool: PgPool,
}

impl FromRef<ApplicationState> for PgPool {
    fn from_ref(state: &ApplicationState) -> PgPool {
        state.pool.clone()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("failed to load .env");

    let pool = pg_pool().await.expect("failed to connect to postgres");

    let listen_address = SocketAddr::from((DEFAULT_LISTEN_HOST, DEFAULT_LISTEN_PORT));

    let state = ApplicationState {
        pool,
    };

    let routes_all = web::routes()
        .with_state(state);

    axum::Server::bind(&listen_address)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn pg_pool() -> core::result::Result<PgPool, sqlx::Error> {
    dotenv().expect("failed to load .env");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be in environment"))
        .await
}