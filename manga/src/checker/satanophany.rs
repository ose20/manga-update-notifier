use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "satanophany";

#[derive(Debug)]
pub struct Satanophany {
    title: String,
    url: String,
    short_title: String,
}

impl Satanophany {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for Satanophany {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;
        driver.goto(url).await?;

        let episode = driver
            .find(By::Css(".mod-episode-title"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(Satanophany);

impl Manga for Satanophany {}
