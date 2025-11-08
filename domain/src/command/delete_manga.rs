use crate::manga::id::MangaId;

#[derive(Debug)]
pub struct DeleteManga {
    pub manga_id: MangaId,
}

impl From<MangaId> for DeleteManga {
    fn from(manga_id: MangaId) -> Self {
        DeleteManga { manga_id }
    }
}
