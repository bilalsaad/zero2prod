use tokio::task::JoinHandle;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s Subscriber.
///
///  Params:
///    - name -> formatting layer's name
///    - env_filter -> env filter string, .e.g. "info" to be used by default.
///
///  
///  Note on return type, we're using `impl Subscriber` as a return type to avoid
///  spelling out a cmplex type. It must be marked as Send and Sync to be used by init subscriber.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // This "weird syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink Implements the MakeWriter trait for all choces
    // of the lifetime paramter `a See https://doc.rust-lang.org/nomicon/hrtb.html
    // for more.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // Dump spans into stdout.
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to init LogTracer");
    set_global_default(subscriber).expect("Failed to set tracing subscriber.");
}

/// Kicks off a tokio blocking task in the current span.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
