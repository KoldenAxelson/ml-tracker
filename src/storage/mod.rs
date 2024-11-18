pub mod database;
pub(crate) mod local;
pub(crate) mod s3;

use crate::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn store_artifact(&self, run_id: Uuid, name: &str, data: &[u8]) -> Result<String>;
    async fn get_artifact(&self, path: &str) -> Result<Vec<u8>>;
    async fn list_artifacts(&self, run_id: Uuid) -> Result<Vec<String>>;
    async fn delete_artifact(&self, path: &str) -> Result<()>;
}

pub use database::Database;
pub use local::LocalStorage;
pub use s3::S3Storage;
