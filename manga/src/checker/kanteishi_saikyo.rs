use anyhow::{anyhow, Result};
use async_trait::async_trait;
use registry::AppRegistry;

use crate::aux::extract_manga_info;

use super::{Manga, MangaCrawler, MangaInfo};

const SHORT_TITLE: &str = "kanteishi_saikyo";
const RSS_URL: &str = "https://pocket.shonenmagazine.com/rss/series/13933686331618251446";

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
    async fn crawl_latest_episode(&self, _url: &str) -> Result<String> {
        let response = reqwest::get(RSS_URL).await?;
        let content = response.bytes().await?;

        let channel = rss::Channel::read_from(&content[..])?;

        if let Some(first_item) = channel.items().first() {
            let title = first_item
                .title
                .clone()
                .ok_or(anyhow!("titleが取得できませんでした"))?;

            Ok(title)
        } else {
            Err(anyhow!("items.firstが存在しない"))
        }
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
