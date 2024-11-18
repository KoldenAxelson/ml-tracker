use crate::{Result, Run, TrackerError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active_run: Option<Uuid>,
}

impl Experiment {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            tags: HashMap::new(),
            created_at: now,
            updated_at: now,
            active_run: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
        self.updated_at = Utc::now();
    }

    pub fn start_run(&mut self) -> Result<Run> {
        if self.active_run.is_some() {
            return Err(TrackerError::InvalidOperation(
                "An active run already exists".to_string(),
            ));
        }

        let run = Run::new(self.id);
        self.active_run = Some(run.id);
        self.updated_at = Utc::now();
        Ok(run)
    }

    pub fn end_run(&mut self, run_id: Uuid) -> Result<()> {
        match self.active_run {
            Some(active_id) if active_id == run_id => {
                self.active_run = None;
                self.updated_at = Utc::now();
                Ok(())
            }
            Some(_) => Err(TrackerError::InvalidOperation(
                "Run ID does not match active run".to_string(),
            )),
            None => Err(TrackerError::InvalidOperation(
                "No active run to end".to_string(),
            )),
        }
    }
}
