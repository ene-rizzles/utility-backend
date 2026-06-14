use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct AdvisoryLock {
    inner: Arc<dashmap::DashMap<String, Arc<Mutex<()>>>>,
}

impl AdvisoryLock {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub async fn lock<Fut, T>(&self, resource: &str, f: impl FnOnce() -> Fut) -> T
    where
        Fut: std::future::Future<Output = T>,
    {
        let mtx = self
            .inner
            .entry(resource.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .value()
            .clone();
        let _guard = mtx.lock().await;
        info!(resource = %resource, "advisory lock acquired");
        f().await
    }
}
