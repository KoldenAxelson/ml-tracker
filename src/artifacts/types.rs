use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub run_id: Uuid,
    pub name: String,
    pub path: String,
    pub metadata: ArtifactMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    pub content_hash: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}
