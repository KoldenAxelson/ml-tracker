use ml_tracker::{Config, ExperimentTracker, Result, RunStatus};
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting distributed training example...");
    let tracker = ExperimentTracker::new(Config::default()).await?;
    let mut experiment = tracker.create_experiment("distributed_training")?;
    let mut run = experiment.start_run()?;
    println!(
        "Created experiment 'distributed_training' with run ID: {}",
        run.id
    );

    let (tx, mut rx) = mpsc::channel(100);

    println!("Spawning {} workers...", 3);
    let mut handles = vec![];
    for worker_id in 0..3 {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            for i in 0..10 {
                let metrics = vec![
                    ("loss", 1.0 / (i + 1) as f64),
                    ("accuracy", 0.6 + (i as f64 * 0.03)),
                ];
                tx.send((worker_id, metrics)).await.unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }));
    }

    drop(tx);

    let mut total_updates = 0;
    while let Some((worker_id, metrics)) = rx.recv().await {
        total_updates += 1;
        println!("Worker {} update:", worker_id);
        for (name, value) in metrics {
            println!("  {}: {:.4}", name, value);
            run.log_metric(format!("worker_{worker_id}.{name}"), value);
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    run.finish(RunStatus::Completed);
    println!(
        "Distributed training completed! Total updates received: {}",
        total_updates
    );
    Ok(())
}
