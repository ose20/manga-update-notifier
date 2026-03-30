use std::time::Duration;

use anyhow::Result;
use application::port::latest_episode_fetcher::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::By;

use crate::fetcher::portal_kind::EpCrawler;

pub struct MangaOneEpCrawler {
    command: FetchLatestEpCommand,
}

impl MangaOneEpCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for MangaOneEpCrawler {
    async fn crawl(&self, driver: &thirtyfour::WebDriver) -> Result<MangaEpisode> {
        // MangaOne 用のクローリングロジックをここに実装する
        driver.goto(self.command.crawl_url.as_str()).await?;

        let scrolling_script = r#"
            // scroll down the page 10 times
            const scrolls = 10
            let scrollCount = 0

            // scroll down and then wait for 0.5s
            const scrollInterval = setInterval(() => {
                window.scrollBy(0, document.body.scrollHeight / 15)
                scrollCount++

                if (scrollCount == scrolls) {
                    clearInterval(scrollInterval)
                }
            }, 500)
        "#;

        driver.execute(scrolling_script, Vec::new()).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;

        let episode = driver
            .find(By::Css("p[class^='line-clamp-1']"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode)?)
    }
}
