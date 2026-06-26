use std::path::PathBuf;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use crate::endpoints::ErrorResponse;
use crate::state::SharedState;

#[derive(Deserialize)]
pub struct ChrootParams {
    new_root: String,
}

#[axum::debug_handler]
pub async fn chroot(
    Query(params): Query<ChrootParams>,
    State(state): State<SharedState>,
) -> Response {
    if !state.read().await.config.enable_chroot {
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "chroot disabled".into(),
            })
        ).into_response();
    }

    let path = PathBuf::from(params.new_root).canonicalize().unwrap();

    if !path.exists() || !path.is_dir() {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not a directory".into(),
            })
        ).into_response();
    }

    state.write().await.config.files_root = path;
    state.write().await.events.notify().await;

    (
        StatusCode::NO_CONTENT,
    ).into_response()
}
