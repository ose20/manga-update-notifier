use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "elder_elite";

#[derive(Debug)]
pub struct ElderElite {
    title: String,
    url: String,
    short_title: String,
}

impl ElderElite {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for ElderElite {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;
        driver.goto(url).await?;

        let episode = driver
            .find(By::Css(".table-list"))
            .await?
            .find(By::Css(".text-bold"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(ElderElite);

impl Manga for ElderElite {}
