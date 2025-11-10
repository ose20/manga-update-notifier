use derive_new::new;
use domain::{
    command::{CreateManga, UpdateManga},
    manga::{
        Manga, MangaEpisode, MangaId, MangaShortTitle, MangaTitle, portal::MangaPortal,
        portal::manga_url::MangaUrl,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMangaRequest {
    pub title: String,
    pub short_title: String,
    pub portal_kind: String,
    pub crawl_url: String,
    pub public_url: String,
}

impl TryFrom<CreateMangaRequest> for CreateManga {
    type Error = anyhow::Error;

    fn try_from(req: CreateMangaRequest) -> Result<Self, Self::Error> {
        let portal_kind = req.portal_kind.parse()?;
        let crawl_url = req.crawl_url.parse()?;
        let public_url = req.public_url.parse()?;
        let portal = MangaPortal::new(portal_kind, crawl_url, public_url)?;
        Ok(CreateManga {
            title: MangaTitle::new(req.title),
            short_title: MangaShortTitle::new(req.short_title),
            portal,
        })
    }
}

// request body に対応
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMangaRequest {
    pub title: String,
    pub short_title: String,
    pub episode: Option<String>,
    pub portal_kind: String,
    pub crawl_url: String,
    pub public_url: String,
}

// path parameter も含めて対応
#[derive(new)]
pub struct UpdateMangaRequestWithId(MangaId, UpdateMangaRequest);
impl TryFrom<UpdateMangaRequestWithId> for UpdateManga {
    type Error = anyhow::Error;

    fn try_from(req_with_id: UpdateMangaRequestWithId) -> Result<Self, Self::Error> {
        let UpdateMangaRequestWithId(
            manga_id,
            UpdateMangaRequest {
                title,
                short_title,
                episode,
                portal_kind,
                crawl_url,
                public_url,
            },
        ) = req_with_id;
        let portal_kind = portal_kind.parse()?;
        let title = MangaTitle::new(title);
        let short_title = MangaShortTitle::new(short_title);
        let episode = episode.map(MangaEpisode::new);
        let crawl_url = crawl_url.parse()?;
        let public_url = public_url.parse()?;
        let portal = MangaPortal::new(portal_kind, crawl_url, public_url)?;
        Ok(UpdateManga {
            manga_id,
            title,
            short_title,
            episode,
            portal,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaResponse {
    pub id: String,
    pub title: String,
    pub short_title: String,
    pub episode: Option<String>,
    pub portal_kind: String,
    pub crawl_url: String,
    pub public_url: String,
}

impl From<Manga> for MangaResponse {
    fn from(manga: Manga) -> Self {
        MangaResponse {
            id: manga.id.to_string(),
            title: manga.title.inner_ref().to_string(),
            short_title: manga.short_title.inner_ref().to_string(),
            episode: manga.episode.map(|ep| ep.inner_ref().to_string()),
            portal_kind: manga.portal.kind().to_string(),
            crawl_url: manga.portal.get_crawl_url().to_string(),
            public_url: manga.portal.get_public_url().to_string(),
        }
    }
}
