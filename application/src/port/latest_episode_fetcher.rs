use anyhow::Result;
use async_trait::async_trait;
use domain::manga::MangaEpisode;

use crate::command::FetchLatestEpCommand;

#[async_trait]
pub trait LatestEpisodeFetcher: Send + Sync {
    async fn fetch_latest_episode(&self, command: FetchLatestEpCommand) -> Result<MangaEpisode>;
}
