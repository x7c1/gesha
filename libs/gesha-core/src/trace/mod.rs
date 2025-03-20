mod message_layer;
use message_layer::MessageLayer;

use std::io;
use std::time::Duration;
use tokio::time::sleep;
use tracing::Subscriber;
use tracing::metadata::LevelFilter;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::{Filter, SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

pub fn init() {
    Registry::default().with(stdout_layer()).init();
}

/// wait for the otel exporter to finish
pub async fn wait_to_export() {
    sleep(duration()).await
}

pub(crate) fn duration() -> Duration {
    Duration::from_millis(100)
}

pub(crate) fn filter_non_otel<S>() -> impl Filter<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    filter_fn(|metadata| !metadata.target().starts_with("opentelemetry"))
}

pub(crate) fn stdout_layer<S>() -> impl Layer<S>
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
