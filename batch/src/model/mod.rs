use kernel::model::manga::{DeleteManga, UpdateMangaPersistence};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MangaExistence {
    pub title: String,
    pub short_title: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaExistenceList {
    pub mangas: Vec<MangaExistence>,
}

impl From<MangaExistence> for UpdateMangaPersistence {
    fn from(value: MangaExistence) -> Self {
        Self {
            title: value.title,
            short_title: value.short_title,
            url: value.url,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MangaAbsence {
    pub short_title: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaAbsenceList {
    pub mangas: Vec<MangaAbsence>,
}

impl From<MangaAbsence> for DeleteManga {
    fn from(value: MangaAbsence) -> Self {
        Self {
            short_title: value.short_title,
        }
    }
}
