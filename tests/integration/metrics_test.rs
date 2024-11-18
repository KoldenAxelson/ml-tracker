use crate::common::TestContext;
use ml_tracker::{MetricLogger, Result};
use rstest::*;
use tokio::time::{sleep, Duration};

#[rstest]
#[tokio::test]
async fn test_metric_batch_logging() -> Result<()> {
    let ctx = TestContext::new().await?;
    let exp = ctx.tracker.create_experiment("test_experiment")?;
    let run = exp.start_run()?;

    let mut logger = MetricLogger::new(
        run.id,
        ctx.tracker.metric_store(),
        MetricLoggerConfig {
            buffer_size: 2,
            flush_interval: Duration::from_secs(1),
        },
    );

    let metrics = vec![("test_metric1", 1.0), ("test_metric2", 2.0)];
    logger.log_batch(metrics).await?;

    let metrics1 = ctx.tracker.get_metrics(run.id, "test_metric1").await?;
    let metrics2 = ctx.tracker.get_metrics(run.id, "test_metric2").await?;

    assert_eq!(metrics1[0].value, 1.0);
    assert_eq!(metrics2[0].value, 2.0);

    Ok(())
}
