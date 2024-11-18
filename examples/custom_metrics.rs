use ml_tracker::{Config, ExperimentTracker, Result, RunStatus};
use std::time::Duration;

struct CustomMetrics {
    gpu_utilization: f64,
    memory_usage: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting custom metrics example...");
    let tracker = ExperimentTracker::new(Config::default()).await?;
    let mut experiment = tracker.create_experiment("custom_metrics_demo")?;
    let mut run = experiment.start_run()?;
    println!(
        "Created experiment 'custom_metrics_demo' with run ID: {}",
        run.id
    );

    for i in 0..10 {
        let metrics = CustomMetrics {
            gpu_utilization: 50.0 + (i as f64 * 0.1),
            memory_usage: 4.0 + (i as f64 * 0.02),
        };

        run.log_metric("gpu_utilization", metrics.gpu_utilization);
        run.log_metric("memory_usage", metrics.memory_usage);
        println!(
            "Iteration {}: GPU Utilization = {:.1}%, Memory Usage = {:.2}GB",
            i, metrics.gpu_utilization, metrics.memory_usage
        );

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    run.finish(RunStatus::Completed);
    println!("Custom metrics tracking completed!");
    Ok(())
}
