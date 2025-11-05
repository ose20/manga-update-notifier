use crate::manga::id::MangaId;

#[derive(Debug)]
pub struct DeleteManga {
    pub manga_id: MangaId,
}
