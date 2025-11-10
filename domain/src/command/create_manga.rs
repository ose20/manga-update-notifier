use crate::manga::{portal::MangaPortal, short_title::MangaShortTitle, title::MangaTitle};

#[derive(Debug)]
pub struct CreateManga {
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub portal: MangaPortal,
}
