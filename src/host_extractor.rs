use std::convert::Infallible;
use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use axum::RequestPartsExt;
use http::header::FORWARDED;
use http::{header, HeaderMap};
use http::request::Parts;
use http::uri::Authority;

const X_FORWARDED_HOST_HEADER_KEY: &str = "X-Forwarded-Host";

/// Extractor that resolves the host of the request.
///
/// Host is resolved through the following, in order:
/// - `Forwarded` header
/// - `X-Forwarded-Host` header
/// - `Host` header
/// - Authority of the request URI
///
/// See <https://www.rfc-editor.org/rfc/rfc9110.html#name-host-and-authority> for the definition of
/// host.
///
/// Note that user agents can set `X-Forwarded-Host` and `Host` headers to arbitrary values so make
/// sure to validate them to avoid security issues.
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

// impl<S> OptionalFromRequestParts<S> for Host
// where
//     S: Send + Sync,
// {
//     type Rejection = Infallible;
// 
//     async fn from_request_parts(
//         parts: &mut Parts,
//         _state: &S,
//     ) -> Result<Option<Self>, Self::Rejection> {
//         if let Some(host) = parse_forwarded(&parts.headers) {
//             return Ok(Some(Host(host.to_owned())));
//         }
// 
//         if let Some(host) = parts
//             .headers
//             .get(X_FORWARDED_HOST_HEADER_KEY)
//             .and_then(|host| host.to_str().ok())
//         {
//             return Ok(Some(Host(host.to_owned())));
//         }
// 
//         if let Some(host) = parts
//             .headers
//             .get(http::header::HOST)
//             .and_then(|host| host.to_str().ok())
//         {
//             return Ok(Some(Host(host.to_owned())));
//         }
// 
//         if let Some(authority) = parts.uri.authority() {
//             return Ok(Some(Host(parse_authority(authority).to_owned())));
//         }
// 
//         Ok(None)
//     }
// }

#[allow(warnings)]
fn parse_forwarded(headers: &HeaderMap) -> Option<&str> {
    // if there are multiple `Forwarded` `HeaderMap::get` will return the first one
    let forwarded_values = headers.get(FORWARDED)?.to_str().ok()?;

    // get the first set of values
    let first_value = forwarded_values.split(',').nth(0)?;

    // find the value of the `host` field
    first_value.split(';').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        key.trim()
            .eq_ignore_ascii_case("host")
            .then(|| value.trim().trim_matches('"'))
    })
}

fn parse_authority(auth: &Authority) -> &str {
    auth.as_str()
        .rsplit('@')
        .next()
        .expect("split always has at least 1 item")
}
