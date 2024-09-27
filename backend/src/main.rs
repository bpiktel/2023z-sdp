mod routing;
mod services;

use axum::extract::DefaultBodyLimit;
use routing::main_route;
use services::{
    database::{migrator::Migrator, surreal::Database},
    file_storage::FileStorage,
    repositories::user::UserRepository,
    runner::run,
};

use crate::services::{app::AppState, config::setup_config, tracing::setup_tracing};

// 64 MiB
const REQUEST_SIZE_LIMIT: usize = 64 * 1024 * 1024;

#[tokio::main]
async fn main() {
    let config = setup_config();
    setup_tracing(&config.tracing);
    let auth_keys = (&config.auth_keys).try_into().expect("Missing PEMs");
    let database = Database::setup(&config.database)
        .await
        .expect("Failed to setup SurrealDB");
    Migrator::new(&config.database.migrations)
        .migrate(&database)
        .await
        .expect("Failed to migrate SurrealDB");
    let file_storage = FileStorage::setup(&config.file_storage)
        .await
        .expect("Failed to setup FileStorage");
    let state = AppState {
        auth_keys,
        database,
        file_storage,
    };

    let repo = UserRepository::new(state.database.clone());
    repo.try_create(&config.admin.username, &config.admin.password)
        .await
        .ok();

    run(
        config.app.endpoint,
        main_route(&config)
            .layer(DefaultBodyLimit::max(REQUEST_SIZE_LIMIT))
            .with_state(state),
    )
    .await;
}
