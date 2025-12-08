use anyhow::Result;
use application::port::latest_episode_fetcher::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

use crate::fetcher::portal_kind::EpCrawler;

pub struct YoungMagazineEpCrawler {
    command: FetchLatestEpCommand,
}

impl YoungMagazineEpCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for YoungMagazineEpCrawler {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        driver.goto(self.command.crawl_url.as_str()).await?;

        let episode = driver
            .find(thirtyfour::By::Css(".mod-episode-title"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode)?)
    }
}
