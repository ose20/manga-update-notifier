use std::str::FromStr;

pub mod portal_kind;

pub struct MangaRow {
    pub manga_id: uuid::Uuid,
    pub title: String,
    pub short_title: String,
    pub url: String,
    pub portal_kind: String,
    pub episode: Option<String>,
}

impl TryFrom<MangaRow> for domain::manga::Manga {
    type Error = anyhow::Error;

    fn try_from(row: MangaRow) -> Result<Self, Self::Error> {
        let id = domain::manga::id::MangaId::from(row.manga_id);
        let title = domain::manga::MangaTitle::new(row.title);
        let short_title = domain::manga::MangaShortTitle::new(row.short_title);
        // Todo: 他にもエラーなところがあったらまとめて教えてあげたい
        let url =
            url::Url::parse(&row.url).map_err(|e| anyhow::anyhow!("Failed to parse URL: {}", e))?;
        let portal_kind = domain::manga::PortalKind::from_str(&row.portal_kind)?;
        let episode = row.episode.map(domain::manga::episode::MangaEpisode::new);

        Ok(domain::manga::Manga {
            id,
            title,
            short_title,
            url,
            portal_kind,
            episode,
        })
    }
}
