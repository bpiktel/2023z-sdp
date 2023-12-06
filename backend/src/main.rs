mod routing;
mod services;

use routing::main_route;
use services::{
    database::{files::FileStorage, surreal::SurrealDb},
    runner::run,
};

use crate::services::{app::AppState, config::setup_config, tracing::setup_tracing};

#[tokio::main]
async fn main() {
    let config = setup_config();
    setup_tracing(&config.tracing);
    let auth_keys = (&config.auth_keys).try_into().expect("Missing PEMs");
    let surreal_db = SurrealDb::setup(&config.surreal_db)
        .await
        .expect("Failed to setup SurrealDb");
    let file_storage = FileStorage::setup(&config.file_storage)
        .await
        .expect("Failed to setup FileStorage");
    let state = AppState {
        auth_keys,
        surreal_db,
        file_storage,
    };
    run(config.app.url, main_route(&config).with_state(state)).await;
}
