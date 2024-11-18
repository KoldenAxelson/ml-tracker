use super::Storage;
use crate::{Result, TrackerError};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn artifact_path(&self, run_id: Uuid, name: &str) -> PathBuf {
        self.root.join(run_id.to_string()).join(name)
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn store_artifact(&self, run_id: Uuid, name: &str, data: &[u8]) -> Result<String> {
        let path = self.artifact_path(run_id, name);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| TrackerError::Storage(e.to_string()))?;
        }

        fs::write(&path, data)
            .await
            .map_err(|e| TrackerError::Storage(e.to_string()))?;

        Ok(path.to_string_lossy().into_owned())
    }

    async fn get_artifact(&self, path: &str) -> Result<Vec<u8>> {
        fs::read(path)
            .await
            .map_err(|e| TrackerError::Storage(e.to_string()))
    }

    async fn list_artifacts(&self, run_id: Uuid) -> Result<Vec<String>> {
        let dir = self.root.join(run_id.to_string());
        let mut artifacts = Vec::new();

        let mut entries = fs::read_dir(&dir)
            .await
            .map_err(|e| TrackerError::Storage(e.to_string()))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| TrackerError::Storage(e.to_string()))?
        {
            artifacts.push(entry.path().to_string_lossy().into_owned());
        }

        Ok(artifacts)
    }

    async fn delete_artifact(&self, path: &str) -> Result<()> {
        fs::remove_file(path)
            .await
            .map_err(|e| TrackerError::Storage(e.to_string()))
    }
}
