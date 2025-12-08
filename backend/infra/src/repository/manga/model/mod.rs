use std::str::FromStr;

use domain::manga::portal::{MangaPortal, portal_kind::PortalKind};

pub mod portal_kind;

pub struct MangaRow {
    pub manga_id: uuid::Uuid,
    pub title: String,
    pub short_title: String,
    pub portal_kind: String,
    pub crawl_url: String,
    pub public_url: String,
    pub episode: Option<String>,
}

impl TryFrom<MangaRow> for domain::manga::Manga {
    type Error = anyhow::Error;

    fn try_from(row: MangaRow) -> Result<Self, Self::Error> {
        let id = domain::manga::id::MangaId::from(row.manga_id);
        let title = domain::manga::MangaTitle::new(row.title)?;
        let short_title = domain::manga::MangaShortTitle::new(row.short_title)?;
        // Todo: 他にもエラーなところがあったらまとめて教えてあげたい
        let crawl_url = url::Url::parse(&row.crawl_url)
            .map_err(|e| anyhow::anyhow!("Failed to parse URL: {}", e))?;
        let public_url = url::Url::parse(&row.public_url)
            .map_err(|e| anyhow::anyhow!("Failed to parse URL: {}", e))?;
        let portal_kind = PortalKind::from_str(&row.portal_kind)
            .map_err(|e| anyhow::anyhow!("Failed to parse PortalKind: {}", e))?;
        let portal = MangaPortal::new(portal_kind, crawl_url, public_url)?;
        let episode = row
            .episode
            .map(domain::manga::episode::MangaEpisode::new)
            .transpose()?;

        Ok(domain::manga::Manga {
            id,
            title,
            short_title,
            portal,
            episode,
        })
    }
}
