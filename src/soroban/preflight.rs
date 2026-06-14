use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreflightResult {
    pub footprint: u64,
    pub instructions: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub recommended_fee: u64,
}

pub async fn run_preflight(
    rpc_url: &str,
    operation_xdr: &str,
) -> Result<PreflightResult, &'static str> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "simulateTransaction",
        "params": {
            "transaction": operation_xdr,
            "resourceConfig": {
                "instructionLeeway": 1000000
            }
        }
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(rpc_url)
        .json(&payload)
        .send()
        .await
        .map_err(|_| "preflight request failed")?;

    let result: PreflightResult = resp.json().await.map_err(|_| "preflight parse failure")?;
    info!(
        footprint = result.footprint,
        instructions = result.instructions,
        "preflight simulation complete"
    );
    Ok(result)
}
