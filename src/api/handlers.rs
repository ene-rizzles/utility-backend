use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::time_series::analytics::{global_engine, DiagnosticReport};

#[derive(Serialize)]
pub struct MeterInfo {
    pub id: String,
    pub tenant_id: String,
    pub location: String,
    pub last_reading: f64,
}

#[derive(Deserialize)]
pub struct ReadingSubmission {
    pub meter_id: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct SettlementRequest {
    pub meter_id: String,
    pub resource_units: f64,
    pub destination_wallet: String,
}

pub async fn list_meters() -> Json<Vec<MeterInfo>> {
    Json(vec![MeterInfo {
        id: "MTR-001".into(),
        tenant_id: "grid-east".into(),
        location: "substation-alpha".into(),
        last_reading: 1234.56,
    }])
}

pub async fn get_meter(Path(id): Path<String>) -> Json<MeterInfo> {
    Json(MeterInfo {
        id,
        tenant_id: "grid-east".into(),
        location: "substation-alpha".into(),
        last_reading: 1234.56,
    })
}

pub async fn list_tariffs() -> Json<Vec<&'static str>> {
    Json(vec![
        "peak:0.15/kWh",
        "off-peak:0.08/kWh",
        "shoulder:0.11/kWh",
    ])
}

pub async fn submit_reading(Json(_body): Json<ReadingSubmission>) -> Json<&'static str> {
    Json("reading accepted")
}

pub async fn settle_account(Json(_body): Json<SettlementRequest>) -> Json<&'static str> {
    Json("settlement initiated")
}

pub async fn get_diagnostics(
    Path(meter_id): Path<String>,
) -> Result<Json<DiagnosticReport>, StatusCode> {
    let mut engine = global_engine()
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine
        .get_diagnostics(&meter_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn metrics_handler() -> &'static str {
    use prometheus::TextEncoder;
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = String::new();
    encoder.encode_utf8(&metric_families, &mut buffer).unwrap();
    Box::leak(buffer.into_boxed_str())
}
