use chrono::{DateTime, Utc};
use tracing::{info, warn};

pub struct DiagnosticResult {
    pub meter_id: String,
    pub expected_volume: f64,
    pub actual_volume: f64,
    pub deviation: f64,
    pub anomaly_detected: bool,
}

pub fn analyze_consumption(
    meter_id: &str,
    readings: &[(DateTime<Utc>, f64)],
    baseline: f64,
    threshold: f64,
) -> DiagnosticResult {
    let actual_volume: f64 = readings.iter().map(|(_, v)| v).sum();
    let deviation = (actual_volume - baseline).abs();
    let anomaly = deviation > threshold;
    if anomaly {
        warn!(
            meter_id,
            deviation,
            threshold,
            "leakage or theft anomaly detected"
        );
    }
    DiagnosticResult {
        meter_id: meter_id.to_string(),
        expected_volume: baseline,
        actual_volume,
        deviation,
        anomaly_detected: anomaly,
    }
}
