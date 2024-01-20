mod message_layer;
use message_layer::MessageLayer;

use opentelemetry::global::set_error_handler;
use opentelemetry::trace::TraceError::ExportFailed;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk as sdk;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use std::fs::File;
use std::io;
use tracing::metadata::LevelFilter;
use tracing::{debug, Subscriber};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

pub fn init() {
    Registry::default()
        .with(stdout_layer())
        .with(trace_layer())
        .with(file_log_layer())
        .init();

    set_error_handler(handle_error).expect("failed to set error handler");
}

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}

fn stdout_layer<S>() -> impl Layer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    MessageLayer::new(io::stdout).with_filter(env_filter)
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
            opentelemetry::KeyValue::new(SERVICE_NAME, "gesha-test"),
        ])))
        .install_batch(sdk::runtime::Tokio)
        .expect("Not running in tokio runtime");

    tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(LevelFilter::INFO)
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
    let layer = layer().pretty().with_writer(file).with_filter(env_filter);
    Some(layer)
}

fn handle_error(err: opentelemetry::global::Error) {
    use opentelemetry::global::Error;
    match err {
        Error::Trace(ExportFailed(e)) => {
            debug!("notes: `docker-compose run` to launch OpenTelemetry collector.");
            debug!("{e:?}");
        }
        _ => opentelemetry::global::handle_error(err),
    }
}
