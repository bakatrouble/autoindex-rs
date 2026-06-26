use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::Json;
use axum::response::{IntoResponse, Response};
use image::{ImageReader};
use serde::Deserialize;
use crate::state::config::Config;
use crate::endpoints::ErrorResponse;
use crate::host_extractor::Host;
use crate::state::SharedState;

#[derive(Deserialize)]
pub struct ThumbnailParams {
    path: Option<String>,
}

#[axum::debug_handler]
pub async fn thumbnail(
    Host(host): Host,
    Query(params): Query<ThumbnailParams>,
    State(state): State<SharedState>,
) -> Response {
    let config = state.read().await.config.clone();
    
    let path = Config::clean_path(params.path.clone().unwrap_or_else(|| ".".to_string()));
    let local_root = match config.get_root_path(&host) {
        Some((root, _)) => root,
        None => {
            return (StatusCode::NOT_FOUND).into_response();
        }
    };
    let local_path = local_root.join(&path);

    if !local_path.exists() || !local_path.is_file() {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not a valid file".into(),
            })
        ).into_response();
    }

    let src_image = ImageReader::open(&local_path);
    if src_image.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "unable to open file".into(),
            })
        ).into_response();
    }
    let src_image = src_image.unwrap().decode();
    if src_image.is_err() {
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "not an image".into(),
            })
        ).into_response();
    }
    let src_image = src_image.unwrap();
    let bytes = state.write().await.thumbnails.get_thumbnail(&src_image);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/webp".parse().unwrap());
    (
        StatusCode::OK,
        headers,
        Vec::from(bytes),
    ).into_response()
}
