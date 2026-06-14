use opentelemetry::{
    global,
    trace::{Span, TracerProvider},
    KeyValue,
};
use tracing::{info, span, Span as TracingSpan};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn init_open_telemetry(service_name: &str) -> anyhow::Result<()> {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(opentelemetry::Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                    KeyValue::new("deployment.environment", "production"),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    global::set_tracer_provider(tracer_provider);
    info!("OpenTelemetry tracing initialized for spatial trace propagation");
    Ok(())
}

pub fn trace_substation_route(substation_id: &str) {
    let span = TracingSpan::current();
    span.set_attribute(KeyValue::new("substation.id", substation_id.to_string()));
    span.set_attribute(KeyValue::new("substation.region", substation_id[..3].to_string()));
}
