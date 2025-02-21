mod message_layer;
use message_layer::MessageLayer;

use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, SdkTracerProvider};
use opentelemetry_sdk::Resource;
use std::fs::File;
use std::io;
use std::time::Duration;
use tokio::time::sleep;
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

/// wait for the otel exporter to finish
pub async fn wait_to_export() {
    sleep(duration()).await
}

fn duration() -> Duration {
    Duration::from_millis(100)
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
    let otel_collector_endpoint = "http://localhost:4317";
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otel_collector_endpoint)
        .build()
        .unwrap();

    let batch_config = BatchConfigBuilder::default()
        .with_scheduled_delay(duration())
        .build();

    let batch_processor = BatchSpanProcessor::builder(exporter)
        .with_batch_config(batch_config)
        .build();

    let service_name = "gesha-verify";
    let provider = SdkTracerProvider::builder()
        .with_span_processor(batch_processor)
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .build();

    let otel_scope_name = "gesha-verify";
    let tracer = provider.tracer(otel_scope_name);

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

    let file_path = "./logs/gesha-verify.log";
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
