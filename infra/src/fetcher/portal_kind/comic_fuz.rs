use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::{By, WebDriver};

use crate::fetcher::portal_kind::EpCrawler;

pub struct ComicFuzCrawler {
    command: FetchLatestEpCommand,
}

impl ComicFuzCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for ComicFuzCrawler {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        // ComicFuz 用のクローリングロジックをここに実装する

        driver.goto(self.command.manga_url.as_str()).await?;

        let episode = driver
            .find(By::Css("h3[class^='Chapter_chapter__name']"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode))
    }
}
