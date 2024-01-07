use std::path::PathBuf;

use serde::Deserialize;
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

pub fn setup_tracing(config: &TracingConfig) {
    let json = config.json.as_ref().map(|json| {
        let writer = rolling::hourly(json, "logs");
        fmt::layer().json().with_writer(writer)
    });
    let console = config.console.then_some(fmt::layer().pretty());
    registry()
        .with(json)
        .with(console)
        .with(EnvFilter::new(&config.directives))
        .init();
}

#[derive(Debug, Deserialize, Clone)]
pub struct TracingConfig {
    directives: String,
    console: bool,
    json: Option<PathBuf>,
}
