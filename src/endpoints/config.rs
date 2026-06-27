use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Response}
};
use serde::Deserialize;
use crate::{
    host_extractor::Host,
    state::SharedState
};

#[derive(Deserialize)]
pub struct ConfigParams {
    pub json: Option<bool>,
}

#[axum::debug_handler]
pub async fn config(
    Host(host): Host,
    State(state): State<SharedState>,
    Query(params): Query<ConfigParams>,
) -> Response {
    let config = state.read().await.config.clone();

    let config_response = config.get_config_response(&host);

    if params.json.is_some() {
        (Json(config_response)).into_response()
    } else {
        let body = String::from("window.initialConfig = ") +
            serde_json::to_string(&config_response).unwrap().as_str() +
            ";";
        Response::builder()
            .header("Content-Type", "application/javascript")
            .body(body)
            .unwrap()
            .into_response()
    }
}
