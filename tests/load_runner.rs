use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::{interval, Duration};
use tracing::{info, warn};

pub struct LoadRunner {
    concurrent_meters: u64,
    meters: Vec<String>,
}

impl LoadRunner {
    pub fn new(count: u64) -> Self {
        let meters: Vec<String> = (0..count)
            .map(|i| format!("MTR-LOAD-{:05}", i))
            .collect();
        Self {
            concurrent_meters: count,
            meters,
        }
    }

    pub async fn run_stress_test(&self) {
        let counter = Arc::new(AtomicU64::new(0));
        let error_counter = Arc::new(AtomicU64::new(0));
        let mut handles = vec![];

        for meter_id in &self.meters {
            let mid = meter_id.clone();
            let cnt = counter.clone();
            let err = error_counter.clone();
            let handle = tokio::spawn(async move {
                let mut tick = interval(Duration::from_millis(100));
                for _ in 0..10 {
                    tick.tick().await;
                    let reading = 200.0 + (rand::random::<f64>() - 0.5) * 50.0;
                    let payload = serde_json::json!({
                        "meter_id": mid,
                        "value": reading,
                        "unit": "kWh",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    });
                    let client = reqwest::Client::new();
                    match client
                        .post("http://localhost:8443/api/v1/readings")
                        .json(&payload)
                        .send()
                        .await
                    {
                        Ok(resp) if resp.status().is_success() => {
                            cnt.fetch_add(1, Ordering::SeqCst);
                        }
                        _ => {
                            err.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                }
            });
            handles.push(handle);
        }

        for h in handles {
            let _ = h.await;
        }

        info!(
            success = counter.load(Ordering::SeqCst),
            errors = error_counter.load(Ordering::SeqCst),
            "load test complete"
        );
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    info!("starting load runner with 100,000 concurrent meters");
    let runner = LoadRunner::new(100_000);
    runner.run_stress_test().await;
}
