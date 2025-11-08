use std::time::Duration;

use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;
use tokio::time::sleep;

use crate::fetcher::portal_kind::EpCrawler;

pub struct TonarinoYJCrawler {
    command: FetchLatestEpCommand,
}

impl TonarinoYJCrawler {
    pub fn new(command: FetchLatestEpCommand) -> Self {
        Self { command }
    }
}

#[async_trait::async_trait]
impl EpCrawler for TonarinoYJCrawler {
    async fn crawl(&self, driver: &WebDriver) -> Result<MangaEpisode> {
        driver.goto(self.command.manga_url.as_str()).await?;
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
        sleep(Duration::from_millis(2000)).await;

        // role="tablist"の要素をXPathで取得
        let tablist = driver
            .find(thirtyfour::By::XPath("//div[@role='tablist']"))
            .await?;

        // tablist内の最初のタブ (1つ目のdiv with role='tab') を取得
        let first_tab = tablist
            .find(thirtyfour::By::XPath(".//div[@role='tab'][1]"))
            .await?;

        // 1つ目のタブをクリック
        first_tab.click().await?;

        sleep(Duration::from_secs(2)).await;

        let wrapper = driver
            .find(thirtyfour::By::Css(
                "div[class^='index-module--series-episode-list-title-wrapper']",
            ))
            .await?;
        let episode = wrapper
            .find(thirtyfour::By::Tag("h4"))
            .await?
            .text()
            .await?;

        Ok(MangaEpisode::new(episode))
    }
}
