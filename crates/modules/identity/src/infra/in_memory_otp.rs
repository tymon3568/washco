use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::OtpStore;
use crate::domain::OtpEntry;

/// In-memory OTP store for MVP. Replace with KeyDB-backed store in production.
#[derive(Clone)]
pub struct InMemoryOtpStore {
    store: Arc<RwLock<HashMap<String, OtpEntry>>>,
}

impl InMemoryOtpStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryOtpStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OtpStore for InMemoryOtpStore {
    async fn store(&self, entry: OtpEntry) -> anyhow::Result<()> {
        self.store
            .write()
            .await
            .insert(entry.phone.clone(), entry);
        Ok(())
    }

    async fn get(&self, phone: &str) -> anyhow::Result<Option<OtpEntry>> {
        Ok(self.store.read().await.get(phone).cloned())
    }

    async fn remove(&self, phone: &str) -> anyhow::Result<()> {
        self.store.write().await.remove(phone);
        Ok(())
    }
}
