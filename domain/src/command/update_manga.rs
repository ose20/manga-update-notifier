use crate::manga::{
    Manga, episode::MangaEpisode, id::MangaId, portal_kind::PortalKind,
    short_title::MangaShortTitle, title::MangaTitle,
};

#[derive(Debug)]
pub struct UpdateManga {
    pub manga_id: MangaId,
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub url: url::Url,
    pub episode: Option<MangaEpisode>,
    pub portal_kind: PortalKind,
}

impl From<Manga> for UpdateManga {
    fn from(manga: Manga) -> Self {
        UpdateManga {
            manga_id: manga.id,
            title: manga.title,
            short_title: manga.short_title,
            url: manga.url,
            episode: manga.episode,
            portal_kind: manga.portal_kind,
        }
    }
}
