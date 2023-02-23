use opentelemetry::{runtime, sdk};
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
    Registry::default()
        .with(trace_layer())
        .with(file_log_layer())
        .with(LevelFilter::INFO)
        .init();
}

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}

fn trace_layer<S>() -> impl Layer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(sdk::trace::config().with_resource(sdk::Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", "gesha-test"),
        ])))
        .install_batch(runtime::Tokio)
        .expect("Not running in tokio runtime");

    tracing_opentelemetry::layer().with_tracer(tracer)
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
