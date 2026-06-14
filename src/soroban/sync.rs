use chrono::{DateTime, Utc};
use tracing::{info, warn};

pub struct LedgerEvent {
    pub event_id: String,
    pub contract_id: String,
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
}

pub struct SlidingWindowSyncer {
    window_start: DateTime<Utc>,
    last_synced_sequence: u64,
}

impl SlidingWindowSyncer {
    pub fn new(window_days: i64) -> Self {
        Self {
            window_start: Utc::now() - chrono::Duration::days(window_days),
            last_synced_sequence: 0,
        }
    }

    pub async fn sync_events(
        &mut self,
        rpc_url: &str,
        contract_id: &str,
    ) -> Result<Vec<LedgerEvent>, &'static str> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getEvents",
            "params": {
                "contractId": contract_id,
                "startLedger": self.last_synced_sequence,
            }
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await
            .map_err(|_| "failed to fetch ledger events")?;

        let events: Vec<LedgerEvent> = resp.json().await.map_err(|_| "failed to parse events")?;
        info!(
            count = events.len(),
            contract = %contract_id,
            "synced ledger events"
        );
        if let Some(last) = events.last() {
            self.last_synced_sequence = last.sequence;
        }
        Ok(events)
    }
}
