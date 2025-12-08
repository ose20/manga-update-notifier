use anyhow::Result;
use async_trait::async_trait;
use domain::manga::{
    Manga, MangaEpisode,
    portal::{manga_url::MangaUrl, portal_kind::PortalKind},
};

#[async_trait]
pub trait LatestEpisodeFetcher: Send + Sync {
    async fn fetch_latest_episode(&self, command: FetchLatestEpCommand) -> Result<MangaEpisode>;
}

#[derive(Debug)]
pub struct FetchLatestEpCommand {
    pub crawl_url: url::Url,
    pub portal_kind: PortalKind,
}

impl From<Manga> for FetchLatestEpCommand {
    fn from(manga: Manga) -> Self {
        FetchLatestEpCommand {
            crawl_url: manga.portal.get_crawl_url().clone(),
            portal_kind: manga.portal.kind(),
        }
    }
}
