pub(crate) mod builder;

use url::Url;
use ring::digest::{digest, Digest, SHA256};

pub(crate) fn generate_digest(url: &Url) -> Digest{
    digest(&SHA256, url.as_str().as_bytes())
}