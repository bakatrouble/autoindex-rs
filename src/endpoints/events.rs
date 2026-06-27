use std::convert::Infallible;
use axum::{
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive}
    }
};
use futures_util::{Stream};
use tokio_stream::{
    StreamExt as _,
    wrappers::{ReceiverStream}
};
use crate::state::SharedState;

#[axum::debug_handler]
pub async fn events(
    State(state): State<SharedState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.write().await.events.add_client().await;

    let stream = ReceiverStream::<String>::new(rx).map(|res| Ok(Event::default().data(res)));

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
}
