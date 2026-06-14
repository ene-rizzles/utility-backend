use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Clone)]
pub struct AdvisoryLock {
    inner: Arc<DashMap<String, Mutex<()>>>,
}

impl AdvisoryLock {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub async fn lock<F, T>(&self, resource: &str, f: F) -> T
    where
        F: Future<Output = T>,
    {
        let entry = self.inner.entry(resource.to_string()).or_insert_with(|| Mutex::new(()));
        let _guard = entry.value().lock().await;
        info!(resource = %resource, "advisory lock acquired");
        f().await
    }
}
