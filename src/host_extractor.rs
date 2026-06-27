use std::convert::Infallible;
use axum::extract::FromRequestParts;
use http::{
    request::Parts,
    uri::Authority
};

#[derive(Debug, Clone)]
pub struct Host(pub axum_extra::headers::Host);

impl<S> FromRequestParts<S> for Host
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let host = parts
            .headers
            .get("host")
            .and_then(|v| v.to_str().ok().map(|v| v.to_string()))
            .unwrap_or_else(|| parts
                .headers
                .get("x-forwarded-host")
                .and_then(|v| v.to_str().ok().map(|v| v.to_string()))
                .unwrap_or(String::from("")));

        Ok(if let Ok(authority) = Authority::from_maybe_shared(host) {
            Self(axum_extra::headers::Host::from(authority))
        } else {
            let authority = Authority::from_maybe_shared("").unwrap();
            Self(axum_extra::headers::Host::from(authority))
        })
    }
}
