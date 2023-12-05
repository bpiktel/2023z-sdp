use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileStorageConfig {
    folder: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileStorage {}

impl FileStorage {
    pub fn setup(config: &FileStorageConfig) -> Self {
        Self {}
    }

    /// Create a file with specific id
    pub async fn create(&self, id: String, data: &[u8]) -> Result<(), ()> {
        todo!();
    }

    /// Get a file with specific id
    pub async fn get(&self, id: String) -> Result<&[u8], ()> {
        todo!();
    }
}
