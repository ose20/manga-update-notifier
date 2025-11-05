use domain::manga::{Manga, PortalKind};

#[derive(Debug)]
pub struct FetchLatestEpCommand {
    pub manga_url: url::Url,
    pub portal_kind: PortalKind,
}

impl From<Manga> for FetchLatestEpCommand {
    fn from(manga: Manga) -> Self {
        FetchLatestEpCommand {
            manga_url: manga.url,
            portal_kind: manga.portal_kind,
        }
    }
}
