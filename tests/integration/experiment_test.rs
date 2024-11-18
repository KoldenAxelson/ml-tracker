use crate::common::TestContext;
use ml_tracker::Result;
use rstest::*;

#[rstest]
#[tokio::test]
async fn test_create_experiment() -> Result<()> {
    let ctx = TestContext::new().await?;

    let exp = ctx.tracker.create_experiment("test_experiment")?;
    assert_eq!(exp.name, "test_experiment");

    let loaded = ctx.tracker.get_experiment(exp.id).await?;
    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap().name, exp.name);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_experiment_run() -> Result<()> {
    let ctx = TestContext::new().await?;
    let exp = ctx.tracker.create_experiment("test_experiment")?;

    let mut run = exp.start_run()?;
    run.log_metric("test_metric", 1.0).await?;
    run.finish()?;

    let metrics = ctx.tracker.get_metrics(run.id, "test_metric").await?;
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].value, 1.0);

    Ok(())
}
