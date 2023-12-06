use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use bytes::{Bytes, BytesMut};
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Deserialize)]
pub struct FileStorageConfig {
    folder: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileStorage {
    folder: PathBuf,
}

impl FileStorage {
    pub async fn setup(config: &FileStorageConfig) -> std::io::Result<Self> {
        tokio::fs::create_dir_all(&config.folder).await?;
        let folder = tokio::fs::canonicalize(&config.folder).await?;
        Ok(Self { folder })
    }

    async fn preprocess_path(&self, path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
        let path = tokio::fs::canonicalize(path).await?;
        if !path.starts_with(&self.folder) || path == self.folder {
            return Err(std::io::Error::new(ErrorKind::NotFound, "File not found"));
        }
        Ok(path)
    }

    /// Create a file with specific id
    pub async fn create(&self, path: String, data: Bytes) -> std::io::Result<()> {
        let path = self.preprocess_path(path).await?;
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .await?;
        file.write_all(&data).await?;
        Ok(())
    }

    /// Get a file with specific id
    pub async fn get(&self, path: String) -> std::io::Result<Bytes> {
        let path = self.preprocess_path(path).await?;
        let mut file = tokio::fs::OpenOptions::new().read(true).open(path).await?;
        let mut buf = BytesMut::new();
        while file.read_buf(&mut buf).await? != 0 {}
        Ok(buf.freeze())
    }
}
