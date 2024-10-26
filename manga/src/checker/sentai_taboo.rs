use std::{thread, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "sentai_taboo";

#[derive(Debug)]
pub struct SentaiTaboo {
    title: String,
    url: String,
    short_title: String,
}

impl SentaiTaboo {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for SentaiTaboo {
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

        thread::sleep(Duration::from_millis(3000));

        let episode = driver
            .find(By::Css(".series-episode-list-title-wrapper"))
            .await?
            .find(By::Tag("h4"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(SentaiTaboo);

impl Manga for SentaiTaboo {}