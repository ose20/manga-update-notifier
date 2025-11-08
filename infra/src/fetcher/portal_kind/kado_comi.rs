use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;

use crate::fetcher::portal_kind::EpCrawler;

pub struct KadoComiEpCrawler {
    command: FetchLatestEpCommand,
}

impl KadoComiEpCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for KadoComiEpCrawler {
    async fn crawl(&self, driver: &thirtyfour::WebDriver) -> Result<MangaEpisode> {
        // KadoComi 用のクローリングロジックをここに実装する
        driver.goto(self.command.manga_url.as_str()).await?;

        let episode = driver
            .find(thirtyfour::By::Css(
                "div[class^='EpisodeThumbnail_titleWrapper']",
            ))
            .await?
            .find(thirtyfour::By::Css("div[class^='EpisodeThumbnail_title']"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode))
    }
}
