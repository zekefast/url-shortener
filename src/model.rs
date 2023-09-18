use ring::digest::Digest;
use serde::Serialize;
use url::Url;
use crate::shortened_url::builder::Builder;

#[derive(Debug)]
pub struct Link {
    pub url: Url,
    pub digest: Digest,
}

impl Link {
    pub(crate) fn new(original: Url, digest: Digest) -> Self {
        Self {
            url: original,
            digest,
        }
    }

    pub(crate) fn shortened_url_builder(&self) -> Builder {
        Builder::with_digest(self.digest)
    }
}