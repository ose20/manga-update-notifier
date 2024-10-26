use std::{thread, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "kanteishi_saikyo";

pub struct KanteishiSaikyo {
    title: String,
    url: String,
    short_title: String,
}

impl KanteishiSaikyo {
    pub async fn try_init(app_registry: &AppRegistry) -> Result<Self> {
        let short_title = SHORT_TITLE;

        let (title, _, url) = extract_manga_info(app_registry, short_title).await?;

        Ok(Self {
            title,
            short_title: short_title.into(),
            url,
        })
    }
}

#[async_trait]
impl MangaCrawler for KanteishiSaikyo {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;

        driver.goto(url).await?;

        let scrolling_script = r#"
        // scroll down the page 10 times
        const scrolls = 10
        let scrollCount = 0

        // scroll down and then wait for 0.5s
        const scrollInterval = setInterval(() => {
            window.scrollBy(0, document.body.scrollHeight / 10)
            scrollCount++

            if (scrollCount == scrolls) {
                clearInterval(scrollInterval)
            }
        }, 500)
    "#;

        driver.execute(scrolling_script, Vec::new()).await?;
        thread::sleep(Duration::from_secs(2));

        let latest_episode = driver
            .find(By::Css(".series-episode-list-title"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(latest_episode)
    }
}

impl MangaInfo for KanteishiSaikyo {
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

impl Manga for KanteishiSaikyo {}
