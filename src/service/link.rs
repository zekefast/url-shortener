use url::Url;
use crate::model::Link;
use crate::shortened_url::generate_digest;


pub fn shorten(url: Url, db: sqlx::PgPool) -> Url {
    let digest = generate_digest(&url);

    Link::new(url, digest)
        .shortened_url_builder()
        .build()
}