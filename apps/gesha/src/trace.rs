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
    // let otel_metrics_layer = tracing_opentelemetry::MetricsLayer::new(build_metrics_controller());

    Registry::default()
        // .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(trace_layer())
        .with(file_log_layer())
        // .with(otel_metrics_layer)
        .with(LevelFilter::INFO)
        .init();
}

pub fn shutdown() {
    opentelemetry::global::shutdown_tracer_provider();
}

// fn build_metrics_controller() -> BasicController {
//     opentelemetry_otlp::new_pipeline()
//         .metrics(
//             opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
//             opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
//             opentelemetry::runtime::Tokio,
//         )
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .tonic()
//                 .with_endpoint("http://localhost:4317"),
//         )
//         .build()
//         .expect("Failed to build metrics controller")
// }

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
        // .with_trace_config(
        //     opentelemetry::sdk::trace::config()
        //         .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
        //         .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
        //         .with_resource(opentelemetry::sdk::Resource::new(vec![opentelemetry::KeyValue::new(
        //             "service.name",
        //             "sample-app",
        //         )]))
        //     ,
        // )
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
