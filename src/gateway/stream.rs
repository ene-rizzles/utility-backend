use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{info, warn};

pub struct MeterEvent {
    pub meter_id: String,
    pub timestamp: i64,
    pub reading: f64,
    pub token_volume: u64,
}

pub struct BackpressureFilter {
    buffer_capacity: usize,
    tx: mpsc::Sender<MeterEvent>,
    rx: mpsc::Receiver<MeterEvent>,
}

impl BackpressureFilter {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<MeterEvent>) {
        let (tx, rx) = mpsc::channel(capacity);
        (
            Self {
                buffer_capacity: capacity,
                tx,
                rx,
            },
            rx,
        )
    }

    pub async fn push(&self, event: MeterEvent) -> Result<(), &'static str> {
        self.tx
            .send(event)
            .await
            .map_err(|_| "backpressure buffer full: dropping event")
    }
}

pub async fn ingest_stream(
    filter: Arc<BackpressureFilter>,
    mut stream: impl tokio_stream::Stream<Item = Result<Bytes, std::io::Error>> + Unpin,
) {
    use tokio_stream::StreamExt;
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(data) => {
                info!(len = data.len(), "received meter datagram");
                let event = MeterEvent {
                    meter_id: String::from("unknown"),
                    timestamp: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
                    reading: 0.0,
                    token_volume: 0,
                };
                if let Err(e) = filter.push(event).await {
                    warn!("{}", e);
                }
            }
            Err(e) => warn!(error = %e, "stream read error"),
        }
    }
}
