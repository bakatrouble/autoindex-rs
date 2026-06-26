use std::time::UNIX_EPOCH;
use axum::extract::{Query, State};
use axum::http::{StatusCode, Uri};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::state::config::Config;
use crate::endpoints::ErrorResponse;
use crate::state::SharedState;

#[derive(Serialize)]
pub struct IndexResponseItem {
    name: String,
    path: String,
    size: u64,
    mime_type: String,
    mtime: u64,
    is_dir: bool,
}

#[derive(Serialize)]
pub struct IndexResponse {
    path: String,
    base_path: Option<String>,
    items: Vec<IndexResponseItem>,
}

#[derive(Deserialize)]
pub struct IndexParams {
    path: Option<String>,
}

#[axum::debug_handler]
pub async fn index(
    uri: Uri,
    Query(params): Query<IndexParams>,
    State(state): State<SharedState>,
) -> Result<Response, Response> {
    let config = state.read().await.config.clone();

    let path = Config::clean_path(params.path.clone().unwrap_or_else(|| ".".to_string()));
    let local_root = match config.get_root_path(&uri) {
        Some((root, _)) => root,
        None => {
            return Err((StatusCode::NOT_FOUND).into_response());
        }
    };
    let local_path = local_root.join(&path);

    if !local_path.exists() || !local_path.is_dir() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "path not found".to_string(),
            })
        ).into_response());
    }

    let items: Vec<IndexResponseItem> = local_path.read_dir().unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            entry.metadata().ok().map(|metadata| IndexResponseItem {
                name: entry.file_name().to_string_lossy().into(),
                path: String::from("/") + &path.join(entry.file_name()).to_string_lossy(),
                size: metadata.len(),
                mime_type: mime_guess::from_path(entry.path()).first_or_octet_stream().to_string(),
                mtime: metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                is_dir: metadata.is_dir(),
            })
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(IndexResponse {
            path: path.to_string_lossy().to_string(),
            base_path: config.expose_base_path
                .then(|| config.files_root.to_string_lossy().to_string()),
            items,
        })
    ).into_response())
}