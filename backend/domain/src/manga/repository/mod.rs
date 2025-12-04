use anyhow::Result;
use async_trait::async_trait;

use crate::manga::{
    Manga, MangaEpisode, MangaId, MangaShortTitle, MangaTitle, portal::MangaPortal,
};

#[async_trait]
pub trait MangaRepository: Send + Sync {
    async fn create_manga(&self, command: CreateManga) -> Result<()>;
    async fn update_manga(&self, command: UpdateManga) -> Result<()>;
    async fn delete_manga(&self, command: DeleteManga) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<Manga>>;
}

#[derive(Debug)]
pub struct CreateManga {
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub portal: MangaPortal,
}

#[derive(Debug)]
pub struct DeleteManga {
    pub manga_id: MangaId,
}

impl From<MangaId> for DeleteManga {
    fn from(manga_id: MangaId) -> Self {
        DeleteManga { manga_id }
    }
}

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
