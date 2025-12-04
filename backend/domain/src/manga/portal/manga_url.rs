use anyhow::Result;
use url::Url;

pub trait MangaUrl {
    fn get_crawl_url(&self) -> &Url;
    fn get_public_url(&self) -> &Url;
}

// crawler が使う url とユーザーが最新話を実際に読む url が同じ場合の型
#[derive(Debug, Clone)]
pub struct SameUrl {
    url: Url,
}

impl SameUrl {
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}

impl MangaUrl for SameUrl {
    fn get_crawl_url(&self) -> &Url {
        &self.url
    }

    fn get_public_url(&self) -> &Url {
        &self.url
    }
}

// crawler が使う url とユーザーが最新話を実際に読む url が異なる場合の型
// RSS を提供している場合など
#[derive(Debug, Clone)]
pub struct SplitUrl {
    crawl: Url,
    public: Url,
}

impl SplitUrl {
    pub fn new(crawl: Url, public: Url) -> Result<Self> {
        if crawl == public {
            return Err(anyhow::anyhow!(
                "Crawl URL and Public URL must be different: {}",
                crawl
            ));
        }
        Ok(Self { crawl, public })
    }
}

impl MangaUrl for SplitUrl {
    fn get_crawl_url(&self) -> &Url {
        &self.crawl
    }

    fn get_public_url(&self) -> &Url {
        &self.public
    }
}
