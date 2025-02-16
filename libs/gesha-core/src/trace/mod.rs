mod message_layer;
use message_layer::MessageLayer;

use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk as sdk;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use std::fs::File;
use std::io;
use tracing::metadata::LevelFilter;
use tracing::Subscriber;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::{Filter, SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

pub fn init() {
    Registry::default()
        .with(stdout_layer())
        .with(trace_layer())
        .with(file_log_layer())
        .with(otel_layer())
        .init();
}

fn filter_non_otel<S>() -> impl Filter<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    filter_fn(|metadata| !metadata.target().starts_with("opentelemetry"))
}

fn stdout_layer<S>() -> impl Layer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    MessageLayer::new(io::stdout)
        .with_filter(filter_non_otel())
        .with_filter(env_filter)
}

fn trace_layer<S>() -> impl Layer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let tracer = SdkTracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint("http://localhost:4317")
                .build()
                .unwrap(),
        )
        .with_resource(
            sdk::Resource::builder()
                .with_attributes(vec![opentelemetry::KeyValue::new(
                    SERVICE_NAME,
                    "gesha-test",
                )])
                .build(),
        )
        .build()
        .tracer("gesha-test");

    tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(LevelFilter::INFO)
        .with_filter(filter_non_otel())
}

fn file_log_layer<S>() -> Option<impl Layer<S>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let env_filter = EnvFilter::builder()
        .with_default_directive("gesha=debug".parse().unwrap())
        .from_env_lossy();

    let file_path = "./logs/gesha-test.log";
    let file = File::create(file_path).expect("unable to create log file");
    let layer = layer()
        .pretty()
        .with_writer(file)
        .with_filter(env_filter)
        .with_filter(filter_non_otel());

    Some(layer)
}

// https://github.com/open-telemetry/opentelemetry-rust/pull/2260
fn otel_layer<S>() -> Option<impl Layer<S>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let layer = layer()
        .with_writer(io::stderr)
        .with_filter(filter_fn(|metadata| {
            metadata.target().starts_with("opentelemetry")
        }));

    Some(layer)
}
