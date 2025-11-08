use derive_new::new;
use domain::{
    command::{CreateManga, UpdateManga},
    manga::{Manga, MangaEpisode, MangaId, MangaShortTitle, MangaTitle},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMangaRequest {
    pub title: String,
    pub short_title: String,
    pub url: String,
    pub portal_kind: String,
}

impl TryFrom<CreateMangaRequest> for CreateManga {
    type Error = anyhow::Error;

    fn try_from(req: CreateMangaRequest) -> Result<Self, Self::Error> {
        let portal_kind = req.portal_kind.parse()?;
        Ok(CreateManga {
            title: MangaTitle::new(req.title),
            short_title: MangaShortTitle::new(req.short_title),
            url: req.url.parse()?,
            portal_kind,
        })
    }
}

// request body に対応
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMangaRequest {
    pub title: String,
    pub short_title: String,
    pub url: String,
    pub episode: Option<String>,
    pub portal_kind: String,
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
                url,
                episode,
                portal_kind,
            },
        ) = req_with_id;
        let portal_kind = portal_kind.parse()?;
        let title = MangaTitle::new(title);
        let short_title = MangaShortTitle::new(short_title);
        let episode = episode.map(MangaEpisode::new);
        let url = url.parse()?;
        Ok(UpdateManga {
            manga_id,
            title,
            short_title,
            url,
            episode,
            portal_kind,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaResponse {
    pub id: String,
    pub title: String,
    pub short_title: String,
    pub url: String,
    pub episode: Option<String>,
    pub portal_kind: String,
}

impl From<Manga> for MangaResponse {
    fn from(manga: Manga) -> Self {
        MangaResponse {
            id: manga.id.to_string(),
            title: manga.title.inner_ref().to_string(),
            short_title: manga.short_title.inner_ref().to_string(),
            url: manga.url.to_string(),
            episode: manga.episode.map(|ep| ep.inner_ref().to_string()),
            portal_kind: manga.portal_kind.to_string(),
        }
    }
}
