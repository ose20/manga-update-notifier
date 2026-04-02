use anyhow::Result;
use application::port::latest_episode_fetcher::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::{By, WebDriver};

use crate::fetcher::portal_kind::EpCrawler;

pub struct GawGawMonster {
    command: FetchLatestEpCommand,
}

impl GawGawMonster {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for GawGawMonster {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        driver.goto(self.command.crawl_url.as_str()).await?;

        let ep = driver
            .find(By::Css(".episode"))
            .await?
            .find(By::Css(".episode__num"))
            .await?
            .text()
            .await?;

        MangaEpisode::new(ep)
    }
}
