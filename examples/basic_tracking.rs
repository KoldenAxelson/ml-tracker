use ml_tracker::{Config, ExperimentTracker, Result, RunStatus};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting basic tracking example...");
    let tracker = ExperimentTracker::new(Config {
        metric_buffer_size: 500,
        metric_flush_interval: Duration::from_secs(30),
        ..Config::default()
    })
    .await?;

    let mut experiment = tracker.create_experiment("mnist_training")?;
    let mut run = experiment.start_run()?;
    println!(
        "Created experiment 'mnist_training' with run ID: {}",
        run.id
    );

    run.set_param("learning_rate", "0.01");
    run.set_param("batch_size", "64");

    for epoch in 0..10 {
        let loss = 2.5 / (epoch + 1) as f64;
        let accuracy = 0.7 + (epoch as f64 * 0.02);

        run.log_metric("loss", loss);
        run.log_metric("accuracy", accuracy);
        println!(
            "Epoch {}: loss = {:.4}, accuracy = {:.4}",
            epoch, loss, accuracy
        );
    }

    run.finish(RunStatus::Completed);
    println!("Training completed successfully!");
    Ok(())
}
