use tower_http::{
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};

// Ensure fallback exists at compile time
const _: &str = include_str!("../../static/index.html");

pub fn static_files_service() -> ServeDir<SetStatus<ServeFile>> {
    ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"))
}
