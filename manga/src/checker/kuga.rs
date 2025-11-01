use anyhow::{anyhow, Result};
use async_trait::async_trait;
use registry::AppRegistry;

use crate::{aux::extract_manga_info, impl_manga_info, impl_try_init};

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "kuga";
const RSS_URL: &str = "https://viewer.heros-web.com/rss/series/10834108156632461068";

#[derive(Debug)]
pub struct Kuga {
    title: String,
    url: String,
    short_title: String,
}

impl Kuga {
    impl_try_init!();
}

#[async_trait]
impl MangaCrawler for Kuga {
    async fn crawl_latest_episode(&self, _url: &str) -> Result<String> {
        let response = {
            let mut retry_cnt = 0;
            loop {
                match reqwest::get(RSS_URL).await {
                    Ok(response) => break response,
                    Err(e) => {
                        retry_cnt += 1;
                        if retry_cnt > 5 {
                            return Err(e.into());
                        } else {
                            continue;
                        }
                    }
                }
            }
        };
        let content = response.bytes().await?;

        let channel = rss::Channel::read_from(&content[..])?;

        if let Some(first_item) = channel.items().first() {
            let title = first_item
                .title
                .clone()
                .ok_or(anyhow!("titleが取得できませんでした"))?;

            Ok(title)
        } else {
            Err(anyhow!("itmes.firstが存在しない"))
        }
    }
}

impl_manga_info!(Kuga);

impl Manga for Kuga {}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::checker::{kuga::Kuga, MangaCrawler};

    #[tokio::test]
    async fn test_crawl() -> Result<()> {
        let kuga = Kuga {
            title: "kuga".into(),
            url: "".into(),
            short_title: "kuga".into(),
        };

        let title = kuga.crawl_latest_episode("dummy").await?;
        println!("title = {title}");

        Ok(())
    }
}
