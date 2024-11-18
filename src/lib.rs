use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

pub mod artifacts;
pub mod experiment;
pub mod experiment_tracker;
pub mod metrics;
pub mod run;
pub mod storage;
pub mod ui;

pub use artifacts::{Artifact, ArtifactManager, ArtifactMetadata};
pub use experiment::Experiment;
pub use experiment_tracker::ExperimentTracker;
pub use metrics::{
    InMemoryMetricStore, MetricLogger, MetricLoggerConfig, MetricPoint, MetricStore, SystemMetrics,
};
pub use run::{Run, RunStatus};
pub use storage::{Database, LocalStorage, S3Storage, Storage};

#[derive(Error, Debug)]
pub enum TrackerError {
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, TrackerError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_path: PathBuf,
    pub database_url: String,
    pub db_pool_size: u32,
    pub db_connection_timeout: Duration,
    pub metric_buffer_size: usize,
    pub metric_flush_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./mltracker"),
            database_url: "sqlite:experiments.db".to_string(),
            db_pool_size: 5,
            db_connection_timeout: Duration::from_secs(30),
            metric_buffer_size: 1000,
            metric_flush_interval: Duration::from_secs(60),
        }
    }
}

impl Experiment {
    pub fn get_active_run(&mut self) -> Result<Option<Run>> {
        // Query for any run that hasn't been marked as completed
        // Implementation details will depend on your storage backend
        todo!("Implement active run retrieval logic")
    }
}
