use super::types::{Artifact, ArtifactMetadata};
use crate::storage::Storage;
use crate::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ArtifactManager {
    storage: Arc<Mutex<dyn Storage>>,
}

impl ArtifactManager {
    pub fn new(storage: Arc<Mutex<dyn Storage>>) -> Self {
        Self { storage }
    }

    pub async fn store(&self, run_id: Uuid, name: &str, data: &[u8]) -> Result<Artifact> {
        let storage = self.storage.lock().await;
        let path = storage.store_artifact(run_id, name, data).await?;

        let metadata = ArtifactMetadata {
            content_hash: blake3::hash(data).to_hex().to_string(),
            size_bytes: data.len() as u64,
            created_at: Utc::now(),
            content_type: None, // Simplified content type handling
            description: None,
            tags: HashMap::new(),
        };

        Ok(Artifact {
            id: Uuid::new_v4(),
            run_id,
            name: name.to_string(),
            path,
            metadata,
        })
    }

    pub async fn get(&self, artifact: &Artifact) -> Result<Vec<u8>> {
        let storage = self.storage.lock().await;
        let data = storage.get_artifact(&artifact.path).await?;

        let hash = blake3::hash(&data);
        if hash.to_hex().to_string() != artifact.metadata.content_hash {
            return Err(crate::TrackerError::InvalidOperation(
                "Artifact content hash mismatch".to_string(),
            ));
        }

        Ok(data)
    }

    pub async fn list(&self, run_id: Uuid) -> Result<Vec<String>> {
        let storage = self.storage.lock().await;
        storage.list_artifacts(run_id).await
    }

    pub async fn delete(&self, artifact: &Artifact) -> Result<()> {
        let storage = self.storage.lock().await;
        storage.delete_artifact(&artifact.path).await
    }
}
