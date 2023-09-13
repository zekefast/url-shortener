#![allow(unused)] // TODO: remove later before release to production.

mod error;
mod prelude;
mod utils;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
