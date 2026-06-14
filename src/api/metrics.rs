use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_histogram_vec, register_gauge, CounterVec, Gauge, HistogramVec,
};

lazy_static! {
    pub static ref GC_PAUSE_SECONDS: Gauge = register_gauge!(
        "utility_gc_pause_seconds",
        "Cumulative GC pause time in seconds"
    )
    .unwrap();
    pub static ref DB_POOL_STARVATION: Gauge = register_gauge!(
        "utility_db_pool_starvation_count",
        "Number of database pool starvation events"
    )
    .unwrap();
    pub static ref INGESTED_EVENTS: CounterVec = register_counter_vec!(
        "utility_ingested_events_total",
        "Total number of ingested meter events",
        &["meter_id", "status"]
    )
    .unwrap();
    pub static ref RPC_LATENCY: HistogramVec = register_histogram_vec!(
        "utility_soroban_rpc_latency_seconds",
        "Soroban RPC call latency in seconds",
        &["method"]
    )
    .unwrap();
    pub static ref ACTIVE_CONNECTIONS: Gauge = register_gauge!(
        "utility_active_gateway_connections",
        "Number of currently active gateway connections"
    )
    .unwrap();
}

pub fn record_ingestion(meter_id: &str, status: &str) {
    INGESTED_EVENTS
        .with_label_values(&[meter_id, status])
        .inc();
}

pub fn record_db_starvation() {
    DB_POOL_STARVATION.inc();
}
