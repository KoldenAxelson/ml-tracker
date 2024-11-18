#![allow(unused_imports)]

use crate::Result;
use async_trait::async_trait;
#[cfg(feature = "s3")]
use aws_sdk_s3::{Client, Config};
use uuid::Uuid;

#[cfg(feature = "s3")]
pub struct S3Storage {
    client: Client,
    bucket: String,
}

#[cfg(feature = "s3")]
impl S3Storage {
    pub async fn new(bucket: impl Into<String>, region: Option<String>) -> Result<Self> {
        let config = Config::builder()
            .region(region.map(aws_sdk_s3::Region::new))
            .build();

        let client = Client::from_conf(config);

        Ok(Self {
            client,
            bucket: bucket.into(),
        })
    }

    fn object_key(&self, run_id: Uuid, name: &str) -> String {
        format!("{}/{}", run_id, name)
    }
}

#[cfg(feature = "s3")]
#[async_trait]
impl Storage for S3Storage {
    async fn store_artifact(&self, run_id: Uuid, name: &str, data: &[u8]) -> Result<String> {
        let key = self.object_key(run_id, name);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(data.to_vec().into())
            .send()
            .await?;

        Ok(key)
    }

    async fn get_artifact(&self, path: &str) -> Result<Vec<u8>> {
        let output = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await?;

        Ok(output.body.collect().await?.to_vec())
    }

    async fn list_artifacts(&self, run_id: Uuid) -> Result<Vec<String>> {
        let prefix = format!("{}/", run_id);

        let objects = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&prefix)
            .send()
            .await?;

        Ok(objects
            .contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|obj| obj.key().map(String::from))
            .collect())
    }

    async fn delete_artifact(&self, path: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(not(feature = "s3"))]
pub struct S3Storage {}

#[cfg(not(feature = "s3"))]
impl S3Storage {
    pub async fn new(_bucket: impl Into<String>, _region: Option<String>) -> Result<Self> {
        Err(crate::TrackerError::Storage(
            "S3 support not enabled".into(),
        ))
    }
}
