use std::path::PathBuf;
use axum::body::Body;
use axum::extract::{State, Request};
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use include_dir::{include_dir, Dir};
use reqwest::Client;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use crate::state::config::Config;
use crate::state::SharedState;

static FRONTEND_DIST_DIR: Dir = include_dir!("frontend/dist");

async fn serve_file_by_path(path: &PathBuf) -> Result<Response, StatusCode> {
    let file = File::open(&path).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let content_length = file
        .metadata()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, content_length)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_response()
    )
}

async fn serve_file(path: &PathBuf, contents: &[u8]) -> Result<Response, StatusCode> {
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();
    let content_length = contents.len();
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, content_length)
        .body(Body::from(contents.to_vec()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_response()
    )
}

#[axum::debug_handler]
pub async fn main_handler(
    uri: Uri,
    State(state): State<SharedState>,
    request: Request,
) -> Result<Response, StatusCode> {
    let config = state.read().await.config.clone();

    let path = Config::clean_path(uri.path());
    let local_root = match config.get_root_path(&uri) {
        Some((root, _)) => root,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    let local_path = local_root.join(&path);
    if local_path.exists() && local_path.is_file() {
        return serve_file_by_path(&local_path).await;
    }

    if let Some(dev_server_url) = config.dev.clone() {
        let client = Client::new();
        let dev_uri = dev_server_url.clone() +
            "/" + path.clone().to_str().unwrap() +
            "?" + uri.clone().query().unwrap_or("");
        let response = client.get(&dev_uri)
            .headers(request.headers().clone())
            .send()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut response_builder = Response::builder()
            .status(response.status());
        *response_builder.headers_mut().unwrap() = response.headers().clone();
        return Ok(response_builder
            .body(Body::from_stream(response.bytes_stream()))
            .unwrap()
        );
    }

    if let Some(assets_file) = FRONTEND_DIST_DIR.get_file(&path) {
        return serve_file(&path, assets_file.contents()).await;
    }

    serve_file(
        &PathBuf::from("index.html"),
        FRONTEND_DIST_DIR.get_file("index.html").unwrap().contents()
    ).await
}
