use std::{thread, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use registry::AppRegistry;
use thirtyfour::By;

use crate::{aux::extract_manga_info, chromedriver::get_driver, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "one_punch_man";

#[derive(Debug)]
pub struct OnePunchMan {
    title: String,
    url: String,
    short_title: String,
}

impl OnePunchMan {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for OnePunchMan {
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
        thread::sleep(Duration::from_millis(2000));

        // role="tablist"の要素をXPathで取得
        let tablist = driver.find(By::XPath("//div[@role='tablist']")).await?;

        // tablist内の最初のタブ (1つ目のdiv with role='tab') を取得
        let first_tab = tablist.find(By::XPath(".//div[@role='tab'][1]")).await?;

        // 1つ目のタブをクリック
        first_tab.click().await?;

        thread::sleep(Duration::from_secs(2));

        let wrapper = driver
            .find(By::Css(
                "div[class^='index-module--series-episode-list-title-wrapper']",
            ))
            .await?;
        let episode = wrapper.find(By::Tag("h4")).await?.text().await?;

        driver.quit().await?;
        Ok(episode)
    }
}

impl_manga_info!(OnePunchMan);

impl Manga for OnePunchMan {}
