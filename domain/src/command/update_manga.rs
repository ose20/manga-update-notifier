use crate::manga::{
    Manga, episode::MangaEpisode, id::MangaId, portal::MangaPortal, short_title::MangaShortTitle,
    title::MangaTitle,
};

#[derive(Debug)]
pub struct UpdateManga {
    pub manga_id: MangaId,
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub portal: MangaPortal,
    pub episode: Option<MangaEpisode>,
}

impl From<Manga> for UpdateManga {
    fn from(manga: Manga) -> Self {
        UpdateManga {
            manga_id: manga.id,
            title: manga.title,
            short_title: manga.short_title,
            episode: manga.episode,
            portal: manga.portal,
        }
    }
}
