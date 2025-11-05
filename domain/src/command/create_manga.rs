use crate::manga::{portal_kind::PortalKind, short_title::MangaShortTitle, title::MangaTitle};

#[derive(Debug)]
pub struct CreateManga {
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub url: url::Url,
    pub portal_kind: PortalKind,
}
