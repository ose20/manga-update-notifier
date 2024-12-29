use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "shujin_tensei";

#[derive(Debug)]
pub struct ShujinTensei {
    title: String,
    url: String,
    short_title: String,
}

impl ShujinTensei {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for ShujinTensei {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;
        driver.goto(url).await?;

        let episode = driver
            .find(By::Css("h3[class^='Chapter_chapter__name']"))
            .await?
            .text()
            .await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(ShujinTensei);

impl Manga for ShujinTensei {}
