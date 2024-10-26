use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "yondome";

#[derive(Debug)]
pub struct Yondome {
    title: String,
    url: String,
    short_title: String,
}

impl Yondome {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for Yondome {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String> {
        let driver = get_driver().await?;
        driver.goto(url).await?;
        let episode = driver
            .find(By::Css("div[class^='EpisodeThumbnail_titleWrapper']"))
            .await?
            .find(By::Css("div[class^='EpisodeThumbnail_title']"))
            .await?
            .text()
            .await?;
        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(Yondome);

impl Manga for Yondome {}
