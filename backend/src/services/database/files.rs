use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use bytes::{Bytes, BytesMut};
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Deserialize)]
pub struct FileStorageConfig {
    pub folder: PathBuf,
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
        let path = path.as_ref();
        let path = path
            .components()
            .last()
            .ok_or(std::io::Error::new(ErrorKind::NotFound, "Not Found"))?;
        let path = self.folder.join(path);
        Ok(path)
    }

    /// Create a file
    pub async fn create(&self, path: impl AsRef<Path>, data: Bytes) -> std::io::Result<()> {
        let path = self.preprocess_path(path).await?;
        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .await?;
        file.write_all(&data).await?;
        Ok(())
    }

    /// Get a file
    pub async fn get(&self, path: impl AsRef<Path>) -> std::io::Result<Bytes> {
        let path = self.preprocess_path(path).await?;
        let mut file = tokio::fs::OpenOptions::new().read(true).open(path).await?;
        let mut buf = BytesMut::new();
        while file.read_buf(&mut buf).await? != 0 {}
        Ok(buf.freeze())
    }

    /// Delete a file
    pub async fn delete(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let path = self.preprocess_path(path).await?;
        tokio::fs::remove_file(path).await?;
        Ok(())
    }
}
