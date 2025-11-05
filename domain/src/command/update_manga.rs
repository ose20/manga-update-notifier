use crate::manga::{
    episode::MangaEpisode, id::MangaId, portal_kind::PortalKind, short_title::MangaShortTitle,
    title::MangaTitle,
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
