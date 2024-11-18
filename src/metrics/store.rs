use crate::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPoint {
    pub run_id: Uuid,
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

#[async_trait]
pub trait MetricStore: Send + Sync {
    fn new() -> Result<Self>
    where
        Self: Sized;
    async fn store_metrics(&self, metrics: &[MetricPoint]) -> Result<()>;
    async fn get_metrics(&self, run_id: Uuid, metric_name: &str) -> Result<Vec<MetricPoint>>;
    async fn get_latest_metric(
        &self,
        run_id: Uuid,
        metric_name: &str,
    ) -> Result<Option<MetricPoint>>;
}

pub struct InMemoryMetricStore {
    metrics: Mutex<Vec<MetricPoint>>,
}

impl InMemoryMetricStore {
    pub fn new() -> Result<Self> {
        Ok(Self {
            metrics: Mutex::new(Vec::new()),
        })
    }
}

#[async_trait]
impl MetricStore for InMemoryMetricStore {
    fn new() -> Result<Self> {
        Self::new()
    }

    async fn store_metrics(&self, metrics: &[MetricPoint]) -> Result<()> {
        let mut store = self.metrics.lock().unwrap();
        store.extend_from_slice(metrics);
        Ok(())
    }

    async fn get_metrics(&self, run_id: Uuid, metric_name: &str) -> Result<Vec<MetricPoint>> {
        let store = self.metrics.lock().unwrap();
        Ok(store
            .iter()
            .filter(|m| m.run_id == run_id && m.name == metric_name)
            .cloned()
            .collect())
    }

    async fn get_latest_metric(
        &self,
        run_id: Uuid,
        metric_name: &str,
    ) -> Result<Option<MetricPoint>> {
        let store = self.metrics.lock().unwrap();
        Ok(store
            .iter()
            .filter(|m| m.run_id == run_id && m.name == metric_name)
            .max_by_key(|m| m.timestamp)
            .cloned())
    }
}
