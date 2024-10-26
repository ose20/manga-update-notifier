use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "toaru_anbu";

#[derive(Debug)]
pub struct ToaruAnbu {
    title: String,
    url: String,
    short_title: String,
}

impl ToaruAnbu {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for ToaruAnbu {
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

impl_manga_info!(ToaruAnbu);

impl Manga for ToaruAnbu {}
