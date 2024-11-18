use crate::metrics::logger::MetricLogger;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use systemstat::{Platform, System};
use tokio::sync::{broadcast, Mutex};

pub struct SystemMetrics {
    logger: Arc<Mutex<MetricLogger>>,
    interval: Duration,
    shutdown: broadcast::Sender<()>,
    is_running: Arc<AtomicBool>,
}

impl SystemMetrics {
    pub fn new(logger: MetricLogger, interval: Duration) -> Self {
        let (shutdown, _) = broadcast::channel(1);
        Self {
            logger: Arc::new(Mutex::new(logger)),
            interval,
            shutdown,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::SeqCst);
        let mut interval = tokio::time::interval(self.interval);
        let mut shutdown = self.shutdown.subscribe();
        let is_running = self.is_running.clone();
        let logger = self.logger.clone();
        let sys = System::new();

        tokio::spawn(async move {
            let result: Result<()> = async {
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let mut metrics = Vec::new();

                            if let Ok(memory) = sys.memory() {
                                metrics.push(("system.memory.used",
                                    (memory.total.as_u64() - memory.free.as_u64()) as f64));
                            }

                            if let Ok(cpu) = sys.cpu_load_aggregate() {
                                if let Ok(cpu_load) = cpu.done() {
                                    metrics.push(("system.cpu.used", (cpu_load.user * 100.0) as f64));
                                }
                            }

                            if !metrics.is_empty() {
                                let mut logger_lock = logger.lock().await;
                                logger_lock.log_batch(metrics).await?;
                            }
                        }
                        _ = shutdown.recv() => {
                            is_running.store(false, Ordering::SeqCst);
                            break;
                        }
                    }
                }
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!("System metrics monitoring error: {}", e);
            }
        });

        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.shutdown.send(());
        Ok(())
    }
}
