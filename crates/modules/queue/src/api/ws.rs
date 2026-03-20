use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Manages per-location broadcast channels for real-time queue updates.
#[derive(Clone)]
pub struct QueueBroadcast {
    channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>,
}

impl QueueBroadcast {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create a broadcast sender for a location.
    pub async fn subscribe(&self, location_id: Uuid) -> broadcast::Receiver<String> {
        let channels = self.channels.read().await;
        if let Some(tx) = channels.get(&location_id) {
            return tx.subscribe();
        }
        drop(channels);

        let mut channels = self.channels.write().await;
        let tx = channels
            .entry(location_id)
            .or_insert_with(|| broadcast::channel(64).0);
        tx.subscribe()
    }

    /// Notify all subscribers of a location that the queue changed.
    pub async fn notify(&self, location_id: Uuid, event: &str) {
        let channels = self.channels.read().await;
        if let Some(tx) = channels.get(&location_id) {
            let _ = tx.send(event.to_string());
        }
    }
}
