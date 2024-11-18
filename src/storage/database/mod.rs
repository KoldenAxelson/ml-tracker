use crate::{Experiment, Result, Run, TrackerError};
use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions},
    Row,
};
use uuid::Uuid;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| TrackerError::Database(e.to_string()))?;

        // Note: migrations should be run manually or via a setup script
        Ok(Self { pool })
    }

    pub async fn create_experiment(&self, experiment: &Experiment) -> Result<()> {
        let tags_json = serde_json::to_string(&experiment.tags)
            .map_err(|e| TrackerError::Database(e.to_string()))?;

        sqlx::query(
            "INSERT INTO experiments (id, name, description, tags, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(experiment.id)
        .bind(&experiment.name)
        .bind(&experiment.description)
        .bind(&tags_json)
        .bind(experiment.created_at)
        .bind(experiment.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| TrackerError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_experiment(&self, id: Uuid) -> Result<Option<Experiment>> {
        let row = sqlx::query(
            "SELECT id, name, description, tags, created_at, updated_at, active_run
            FROM experiments WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| TrackerError::Database(e.to_string()))?;

        if let Some(row) = row {
            let tags: serde_json::Value = serde_json::from_str(
                row.try_get("tags")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
            )
            .map_err(|e| TrackerError::Database(e.to_string()))?;

            Ok(Some(Experiment {
                id: row
                    .try_get("id")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                name: row
                    .try_get("name")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                description: row
                    .try_get("description")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                tags: serde_json::from_value(tags)
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                created_at: row
                    .try_get("created_at")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                updated_at: row
                    .try_get("updated_at")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
                active_run: row
                    .try_get("active_run")
                    .map_err(|e| TrackerError::Database(e.to_string()))?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_run(&self, run: &Run) -> Result<()> {
        sqlx::query(
            "INSERT INTO runs (
                id, experiment_id, status, metrics, params, 
                tags, artifacts, start_time, end_time
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(run.id)
        .bind(run.experiment_id)
        .bind(
            serde_json::to_string(&run.status)
                .map_err(|e| TrackerError::Database(e.to_string()))?,
        )
        .bind(
            serde_json::to_string(&run.metrics)
                .map_err(|e| TrackerError::Database(e.to_string()))?,
        )
        .bind(
            serde_json::to_string(&run.params)
                .map_err(|e| TrackerError::Database(e.to_string()))?,
        )
        .bind(serde_json::to_string(&run.tags).map_err(|e| TrackerError::Database(e.to_string()))?)
        .bind(
            serde_json::to_string(&run.artifacts)
                .map_err(|e| TrackerError::Database(e.to_string()))?,
        )
        .bind(run.start_time)
        .bind(run.end_time)
        .execute(&self.pool)
        .await
        .map_err(|e| TrackerError::Database(e.to_string()))?;

        Ok(())
    }
}
