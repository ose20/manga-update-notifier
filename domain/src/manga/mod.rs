pub mod episode;
pub mod id;
pub mod portal_kind;
pub mod repository;
pub mod short_title;
pub mod title;

pub use episode::MangaEpisode;
pub use id::MangaId;
pub use portal_kind::PortalKind;
pub use short_title::MangaShortTitle;
pub use title::MangaTitle;

#[derive(Debug)]
pub struct Manga {
    pub id: MangaId,
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub url: url::Url,
    pub episode: Option<MangaEpisode>,
    pub portal_kind: PortalKind,
}
