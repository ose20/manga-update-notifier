use anyhow::Result;
use domain::manga::MangaEpisode;
use url::Url;

pub async fn crawl_ep_from_rss(url: Url) -> Result<MangaEpisode> {
    let response = {
        let mut retry_count = 0;
        loop {
            match reqwest::get(url.as_str()).await {
                Ok(resp) => break resp,
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= 3 {
                        return Err(anyhow::anyhow!(
                            "Failed to fetch page after 3 retries: {}",
                            e
                        ));
                    }
                }
            }
        }
    };

    let body = response.text().await?;
    let channel = rss::Channel::read_from(body.as_bytes())?;
    if let Some(first_item) = channel.items().first() {
        if let Some(title) = first_item.title() {
            Ok(MangaEpisode::new(title.to_string())?)
        } else {
            Err(anyhow::anyhow!("No title found in the first RSS item"))
        }
    } else {
        Err(anyhow::anyhow!("No items found in the RSS feed"))
    }
}
