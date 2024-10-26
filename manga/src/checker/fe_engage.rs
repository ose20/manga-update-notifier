use std::{thread, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "fe_engage";

#[derive(Debug)]
pub struct FeEngage {
    title: String,
    url: String,
    short_title: String,
}

impl FeEngage {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for FeEngage {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;
        driver.goto(url).await?;

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
        thread::sleep(Duration::from_secs(2));

        let episode = driver
            .find(By::Css(".series-episode-list-title"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(FeEngage);

impl Manga for FeEngage {}
