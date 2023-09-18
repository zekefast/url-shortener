use ring::digest::Digest;
use url::Url;
use base64::engine::{Engine, general_purpose::URL_SAFE_NO_PAD};

pub const DEFAULT_SCHEME:&str = "http";
pub const DEFAULT_TLD: &str = "tier.app";

fn base_url() -> Url {
    Url::parse(&format!("{DEFAULT_SCHEME}://{DEFAULT_TLD}"))
        .expect("Can't parse base for shortened URL. It is likely something with your code!")
}

fn encode_digest(digest: Digest) -> String {
    URL_SAFE_NO_PAD.encode(digest)
}

pub(crate) struct Builder {
    url: Url,
    digest: Option<Digest>,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            url: base_url(),
            digest: None,
        }
    }

    pub(crate) fn with_digest(digest: Digest) -> Self {
        Self {
            url: base_url(),
            digest: Some(digest),
        }
    }

    pub(crate) fn with_scheme(&mut self, scheme: &str) -> &mut Self {
        self.url.set_scheme(scheme);

        self
    }

    pub(crate) fn with_tld(&mut self, tld: &str) -> &mut Self {
        self.url.set_host(Some(tld));

        self
    }

    pub(crate) fn build(self) -> Url {
        let Self {mut url, digest } = self;

        if let Some(digest) = digest {
            url.set_path(&encode_digest(digest));
        }

        url
    }
}