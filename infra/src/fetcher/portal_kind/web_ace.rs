use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

use crate::fetcher::portal_kind::EpCrawler;

pub struct WebAceEpCrawler {
    command: FetchLatestEpCommand,
}

impl WebAceEpCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for WebAceEpCrawler {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        driver.goto(self.command.manga_url.as_str()).await?;

        let episode = driver
            .find(thirtyfour::By::Css(
                "li.table-view-cell.media .media-body .text-bold",
            ))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode))
    }
}
