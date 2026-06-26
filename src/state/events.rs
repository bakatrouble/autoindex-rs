use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::debug;
use tracing::log::info;

pub struct Events {
    subscribers: Mutex<Vec<Sender<String>>>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
        }
    }

    pub async fn add_client(&mut self) -> Receiver<String> {
        let (tx, rx) = mpsc::channel(10);
        tx.send("welcome".into()).await.unwrap();

        self.subscribers.lock().await.push(tx);

        debug!("created SSE channel");

        rx
    }

    pub async fn notify(&mut self) {
        info!("notifying clients about config update");
        let clients = self.subscribers.lock().await.clone();
        let send_futures = clients.iter().map(|client| client.send("update".into()));

        let results = futures_util::future::join_all(send_futures).await;
        let mut results_iter = results.iter();
        let mut lock = self.subscribers.lock().await;
        lock.retain(|_| results_iter.next().unwrap().is_ok());
        info!("removed {} clients", clients.len() - lock.len())
    }

    pub async fn kill_all_connections(&self) {
        self.subscribers.lock().await.clear();
    }
}
