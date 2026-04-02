use anyhow::Result;
use application::port::latest_episode_fetcher::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

use crate::fetcher::{portal_kind::EpCrawler, rss_fetcher::crawl_ep_from_rss};

pub struct ComicDaysCrawler {
    command: FetchLatestEpCommand,
}

impl ComicDaysCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for ComicDaysCrawler {
    async fn crawl(&self, _driver: &WebDriver) -> Result<MangaEpisode> {
        // ComicDays 用のクローリングロジックをここに実装する
        crawl_ep_from_rss(self.command.crawl_url.clone()).await
    }
}
