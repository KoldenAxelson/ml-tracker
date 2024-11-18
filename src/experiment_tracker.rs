use crate::metrics::store::{InMemoryMetricStore, MetricStore};
use crate::{Config, Experiment, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ExperimentTracker {
    config: Config,
    store: Arc<Mutex<dyn MetricStore>>,
}

impl ExperimentTracker {
    pub async fn new(config: Config) -> Result<Self> {
        let store = Arc::new(Mutex::new(InMemoryMetricStore::new()?));

        Ok(Self { config, store })
    }

    pub fn create_experiment(&self, name: impl Into<String>) -> Result<Experiment> {
        Ok(Experiment::new(name))
    }

    pub fn metric_store(&self) -> Arc<Mutex<dyn MetricStore>> {
        self.store.clone()
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    pub async fn get_experiment(&self, _id: Uuid) -> Result<Option<Experiment>> {
        todo!("Implement experiment retrieval")
    }

    pub async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
