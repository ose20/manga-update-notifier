use anyhow::Result;
use async_trait::async_trait;
use domain::event;

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn notify_latest_episode(&self, event: event::DetectLastEpUpdated) -> Result<()>;

    async fn notify_error(&self, event: event::DetectFetchError) -> Result<()>;
}
