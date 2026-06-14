use tokio::sync::Mutex;
use tracing::{info, warn};

pub enum TxStatus {
    Pending,
    Committed,
    RolledBack,
}

#[allow(dead_code)]
pub struct TwoPhaseCommit {
    tx_id: String,
    status: TxStatus,
    db_state_backup: Option<serde_json::Value>,
    onchain_state_backup: Option<serde_json::Value>,
}

pub struct TxStateController {
    active_transactions: Mutex<Vec<TwoPhaseCommit>>,
}

impl TxStateController {
    pub fn new() -> Self {
        Self {
            active_transactions: Mutex::new(Vec::new()),
        }
    }

    pub async fn begin(&self, tx_id: String) {
        let tx = TwoPhaseCommit {
            tx_id,
            status: TxStatus::Pending,
            db_state_backup: None,
            onchain_state_backup: None,
        };
        self.active_transactions.lock().await.push(tx);
        info!("two-phase commit transaction started");
    }

    pub async fn commit(&self, tx_id: &str) -> Result<(), &'static str> {
        let mut txs = self.active_transactions.lock().await;
        if let Some(tx) = txs.iter_mut().find(|t| t.tx_id == tx_id) {
            tx.status = TxStatus::Committed;
            info!(tx = %tx_id, "transaction committed");
            Ok(())
        } else {
            Err("transaction not found")
        }
    }

    pub async fn rollback(&self, tx_id: &str) -> Result<(), &'static str> {
        let mut txs = self.active_transactions.lock().await;
        if let Some(tx) = txs.iter_mut().find(|t| t.tx_id == tx_id) {
            tx.status = TxStatus::RolledBack;
            warn!(tx = %tx_id, "transaction rolled back");
            Ok(())
        } else {
            Err("transaction not found")
        }
    }
}
