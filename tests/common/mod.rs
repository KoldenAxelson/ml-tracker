use ml_tracker::{Config, ExperimentTracker, Result, RunStatus};

#[tokio::test]
async fn test_basic_experiment_tracking() -> Result<()> {
    let tracker = ExperimentTracker::new(Config::default()).await?;
    let mut experiment = tracker.create_experiment("test_experiment")?;
    let mut run = experiment.start_run()?;

    // Test parameters
    run.set_param("learning_rate", "0.01");
    run.set_param("batch_size", "32");

    // Test metrics
    run.log_metric("loss", 0.5);
    run.log_metric("accuracy", 0.95);

    run.finish(RunStatus::Completed);
    Ok(())
}

#[tokio::test]
async fn test_multiple_runs() -> Result<()> {
    let tracker = ExperimentTracker::new(Config::default()).await?;
    let mut experiment = tracker.create_experiment("multi_run_test")?;

    for i in 0..3 {
        let mut run = experiment.start_run()?;
        run.set_param("run_number", i.to_string());
        run.log_metric("value", i as f64);
        run.finish(RunStatus::Completed);
    }

    Ok(())
}

#[tokio::test]
async fn test_cleanup_active_run() -> Result<()> {
    let tracker = ExperimentTracker::new(Config::default()).await?;
    let mut experiment = tracker.create_experiment("cleanup_test")?;

    // Try to get and end any active run
    if let Some(mut active_run) = experiment.get_active_run()? {
        active_run.finish(RunStatus::Completed);
    }

    // Now try a normal run
    let mut run = experiment.start_run()?;
    run.set_param("test", "value");
    run.finish(RunStatus::Completed);

    Ok(())
}
