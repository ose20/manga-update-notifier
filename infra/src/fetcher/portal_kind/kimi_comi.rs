use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

use crate::fetcher::portal_kind::EpCrawler;

pub struct KimiComiEpCrawler {
    command: FetchLatestEpCommand,
}

impl KimiComiEpCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for KimiComiEpCrawler {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        driver.goto(self.command.manga_url.as_str()).await?;

        let episode = driver
            .find(thirtyfour::By::Css(".series-ep-list-item-h-text"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode))
    }
}
