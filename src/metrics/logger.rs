use crate::metrics::store::{MetricPoint, MetricStore};
use crate::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MetricLoggerConfig {
    pub buffer_size: usize,
    pub flush_interval: std::time::Duration,
}

impl Default for MetricLoggerConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            flush_interval: std::time::Duration::from_secs(60),
        }
    }
}

pub struct MetricLogger {
    run_id: Uuid,
    store: Arc<Mutex<dyn MetricStore>>,
    buffer: Vec<MetricPoint>,
    config: MetricLoggerConfig,
    shutdown: broadcast::Sender<()>,
}

impl MetricLogger {
    pub fn new(
        run_id: Uuid,
        store: Arc<Mutex<dyn MetricStore>>,
        config: MetricLoggerConfig,
    ) -> Self {
        let (shutdown, _) = broadcast::channel(1);
        Self {
            run_id,
            store,
            buffer: Vec::with_capacity(config.buffer_size),
            config,
            shutdown,
        }
    }

    pub async fn log(&mut self, name: impl Into<String>, value: f64) -> Result<()> {
        let point = MetricPoint {
            run_id: self.run_id,
            name: name.into(),
            value,
            timestamp: Utc::now(),
        };

        self.buffer.push(point);

        if self.buffer.len() >= self.config.buffer_size {
            self.flush().await?;
        }

        Ok(())
    }

    pub async fn log_batch(&mut self, metrics: Vec<(&str, f64)>) -> Result<()> {
        let timestamp = Utc::now();
        let points: Vec<_> = metrics
            .into_iter()
            .map(|(name, value)| MetricPoint {
                run_id: self.run_id,
                name: name.to_string(),
                value,
                timestamp,
            })
            .collect();

        self.buffer.extend(points);

        if self.buffer.len() >= self.config.buffer_size {
            self.flush().await?;
        }

        Ok(())
    }

    pub async fn flush(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let points = std::mem::take(&mut self.buffer);
        let store = self.store.lock().await;
        store.store_metrics(&points).await?;

        Ok(())
    }

    pub async fn start_auto_flush(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(self.config.flush_interval);
        let mut shutdown = self.shutdown.subscribe();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if !self.buffer.is_empty() {
                        self.flush().await?;
                    }
                }
                _ = shutdown.recv() => {
                    if !self.buffer.is_empty() {
                        self.flush().await?;
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.shutdown.send(());
        Ok(())
    }
}
