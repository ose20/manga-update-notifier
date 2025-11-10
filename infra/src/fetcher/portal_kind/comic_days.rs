use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

use crate::fetcher::portal_kind::EpCrawler;

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
        let response = {
            let mut retry_count = 0;
            loop {
                match reqwest::get(self.command.crawl_url.as_str()).await {
                    Ok(resp) => break resp,
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= 3 {
                            return Err(anyhow::anyhow!(
                                "Failed to fetch page after 3 retries: {}",
                                e
                            ));
                        }
                    }
                }
            }
        };

        let body = response.text().await?;
        let channel = rss::Channel::read_from(body.as_bytes())?;
        if let Some(first_item) = channel.items().first() {
            if let Some(title) = first_item.title() {
                Ok(MangaEpisode::new(title.to_string()))
            } else {
                Err(anyhow::anyhow!("No title found in the first RSS item"))
            }
        } else {
            Err(anyhow::anyhow!("No items found in the RSS feed"))
        }
    }
}
