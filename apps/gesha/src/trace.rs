use opentelemetry_otlp::WithExportConfig;
use std::fs::File;
use tracing::metadata::LevelFilter;
use tracing::Subscriber;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

pub fn init() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
            opentelemetry::sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                "gesha-test",
            )]),
        ))
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Not running in tokio runtime");

    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // let otel_metrics_layer = tracing_opentelemetry::MetricsLayer::new(build_metrics_controller());

    Registry::default()
        // .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(otel_trace_layer)
        .with(file_log_layer())
        // .with(otel_metrics_layer)
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}

fn file_log_layer<S>() -> Option<impl Layer<S>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_path = "./logs/gesha-test.log";
    let file = File::create(file_path).expect("unable to create log file");
    let layer = layer().pretty().with_writer(file).with_filter(env_filter);
    Some(layer)
}
