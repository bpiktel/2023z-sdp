mod routing;
mod services;

use axum::extract::FromRef;
use routing::main_route;
use services::{
    database::{files::FileStorage, repositories::user::UserRepository, surreal::SurrealDb},
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

    let repo = UserRepository::new(state.surreal_db.clone());
    repo.try_create("root", "root").await.ok();

    run(config.app.url, main_route(&config).with_state(state)).await;
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;

    use crate::routing;
    use crate::services;
    use utoipa::OpenApi;

    #[test]
    fn make_openapi_json() {
        #[derive(OpenApi)]
        #[openapi(
            paths(
                routing::api::audio::create_audio,
                routing::api::audio::delete_audio,
                routing::api::audio::get_audio,
            ),
            components(
                schemas(
                    services::database::repositories::sample::SampleInfo,
                )
            ),
            tags(
                (name = "todo", description = "chghckgj")
            )
        )]
        pub struct ApiDoc;
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("openapi.json")
            .unwrap();
        serde_json::to_writer_pretty(file, &ApiDoc::openapi()).unwrap();
    }
}
