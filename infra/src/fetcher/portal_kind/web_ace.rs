use anyhow::Result;
use application::command::FetchLatestEpCommand;
use domain::manga::MangaEpisode;
use reqwest::Client;
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
    async fn crawl(&self, _driver: &WebDriver) -> Result<MangaEpisode> {
        // WebAce の RSS 取得には User-Agent ヘッダーが必要
        // Todo: rss 使うポータルは抽象化できる
        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0",
            )
            .build()?;
        let response = {
            let mut retry_count = 0;
            loop {
                match client.get(self.command.crawl_url.as_str()).send().await {
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
