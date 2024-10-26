use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "degarashi";

#[derive(Debug)]
pub struct Degarashi {
    title: String,
    url: String,
    short_title: String,
}

impl Degarashi {
    pub async fn try_init(app_registry: &AppRegistry) -> Result<Self> {
        let short_title = SHORT_TITLE.to_string();

        let (title, _, url) = extract_manga_info(app_registry, &short_title).await?;

        Ok(Self {
            title,
            short_title,
            url,
        })
    }
}

#[async_trait]
impl MangaCrawler for Degarashi {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;

        driver.goto(url).await?;

        let latest_episode = driver
            .find(By::Css("li.table-view-cell.media .media-body .text-bold"))
            .await?;

        let episode = latest_episode.text().await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl MangaInfo for Degarashi {
    fn short_title(&self) -> &str {
        &self.short_title
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl Manga for Degarashi {}

#[cfg(test)]
mod test {
    use super::*;

    // chromedriver を 9515 で動かしていることが前提
    #[tokio::test]
    async fn test() {
        let url = "https://web-ace.jp/youngaceup/contents/1000137/";
        let degarashi = Degarashi {
            title: "dummy".into(),
            short_title: "dummy".into(),
            url: "dummy".into(),
        };

        let episode = degarashi.crawl_latest_episode(url).await;
        assert!(episode.is_ok());
        println!("episode: {}", episode.unwrap());
    }
}
