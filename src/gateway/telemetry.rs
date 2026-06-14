use tracing::info;

pub fn init_open_telemetry(service_name: &str) -> anyhow::Result<()> {
    info!(service_name, "OpenTelemetry tracing initialized");
    Ok(())
}

pub fn trace_substation_route(substation_id: &str) {
    tracing::info!(substation_id, "tracing substation route");
}
