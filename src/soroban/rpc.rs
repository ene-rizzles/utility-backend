use backoff::{ExponentialBackoff, future::retry};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct SorobanRpcResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

pub struct CircuitBreaker {
    failure_count: u64,
    max_failures: u64,
    is_open: bool,
}

impl CircuitBreaker {
    pub fn new(max_failures: u64) -> Self {
        Self {
            failure_count: 0,
            max_failures,
            is_open: false,
        }
    }

    pub async fn call_rpc(
        &mut self,
        rpc_url: &str,
        payload: serde_json::Value,
    ) -> Result<SorobanRpcResponse, &'static str> {
        if self.is_open {
            return Err("circuit breaker open: rpc calls suspended");
        }

        let op = || async {
            let client = reqwest::Client::new();
            let resp = client
                .post(rpc_url)
                .json(&payload)
                .send()
                .await
                .map_err(|_| backoff::Error::Transient {
                    err: "rpc request failed",
                    retry_after: None,
                })?;
            let body: SorobanRpcResponse = resp.json().await.map_err(|_| backoff::Error::Permanent {
                err: "failed to parse rpc response",
            })?;
            Ok(body)
        };

        let backoff = ExponentialBackoff::default();
        match retry(backoff, op).await {
            Ok(resp) => {
                self.failure_count = 0;
                self.is_open = false;
                info!("soroban rpc call succeeded");
                Ok(resp)
            }
            Err(_) => {
                self.failure_count += 1;
                if self.failure_count >= self.max_failures {
                    self.is_open = true;
                    warn!("circuit breaker opened due to repeated rpc failures");
                }
                Err("rpc call failed after retries")
            }
        }
    }
}
