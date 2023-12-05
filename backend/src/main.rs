mod routing;
mod services;

use routing::main_route;
use services::runner::run;

use crate::services::{app::AppState, config::setup_config, tracing::setup_tracing};

#[tokio::main]
async fn main() {
    let config = setup_config();
    setup_tracing(&config.tracing);
    let auth_keys = (&config.auth_keys).try_into().expect("Missing PEMs");
    let state = AppState { auth_keys };
    run(config.app.url, main_route(&config).with_state(state)).await;
}
