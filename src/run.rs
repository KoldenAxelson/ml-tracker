use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunStatus {
    Running,
    Completed,
    Failed,
    Interrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: Uuid,
    pub experiment_id: Uuid,
    pub status: RunStatus,
    pub metrics: HashMap<String, Vec<(DateTime<Utc>, f64)>>,
    pub params: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    pub artifacts: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Run {
    pub fn new(experiment_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            experiment_id,
            status: RunStatus::Running,
            metrics: HashMap::new(),
            params: HashMap::new(),
            tags: HashMap::new(),
            artifacts: Vec::new(),
            start_time: Utc::now(),
            end_time: None,
        }
    }

    pub fn log_metric(&mut self, name: impl Into<String>, value: f64) {
        let timestamp = Utc::now();
        self.metrics
            .entry(name.into())
            .or_default()
            .push((timestamp, value));
    }

    pub fn set_param(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.params.insert(key.into(), value.into());
    }

    pub fn add_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }

    pub fn add_artifact(&mut self, path: impl Into<String>) {
        self.artifacts.push(path.into());
    }

    pub fn finish(&mut self, status: RunStatus) {
        self.status = status;
        self.end_time = Some(Utc::now());
    }
}
