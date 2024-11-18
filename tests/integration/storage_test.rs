use crate::common::TestContext;
use ml_tracker::Result;
use rstest::*;
use uuid::Uuid;

#[rstest]
#[tokio::test]
async fn test_artifact_storage() -> Result<()> {
    let ctx = TestContext::new().await?;
    let run_id = Uuid::new_v4();

    let test_data = b"test artifact data";
    let path = ctx
        .tracker
        .storage()
        .store_artifact(run_id, "test.txt", test_data)
        .await?;

    let loaded = ctx.tracker.storage().get_artifact(&path).await?;

    assert_eq!(loaded, test_data);

    Ok(())
}
