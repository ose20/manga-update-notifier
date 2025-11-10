use domain::manga::{
    Manga,
    portal::{manga_url::MangaUrl, portal_kind::PortalKind},
};

#[derive(Debug)]
pub struct FetchLatestEpCommand {
    pub crawl_url: url::Url,
    pub portal_kind: PortalKind,
}

impl From<Manga> for FetchLatestEpCommand {
    fn from(manga: Manga) -> Self {
        FetchLatestEpCommand {
            crawl_url: manga.portal.get_crawl_url().clone(),
            portal_kind: manga.portal.kind(),
        }
    }
}
