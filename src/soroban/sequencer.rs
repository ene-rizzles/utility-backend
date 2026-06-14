use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;
use tracing::info;

pub struct NonceSequencer {
    grid_nonce: AtomicU64,
    last_committed: Mutex<u64>,
}

impl NonceSequencer {
    pub fn new() -> Self {
        Self {
            grid_nonce: AtomicU64::new(0),
            last_committed: Mutex::new(0),
        }
    }

    pub fn next_nonce(&self, grid_id: &str) -> u64 {
        let nonce = self.grid_nonce.fetch_add(1, Ordering::SeqCst);
        info!(grid = %grid_id, nonce, "nonce issued");
        nonce
    }

    pub async fn commit_nonce(&self, nonce: u64) -> Result<(), &'static str> {
        let mut last = self.last_committed.lock().await;
        if nonce <= *last {
            return Err("nonce already committed: possible double-spend");
        }
        *last = nonce;
        Ok(())
    }
}
