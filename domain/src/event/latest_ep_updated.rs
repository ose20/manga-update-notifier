use crate::manga::{MangaEpisode, MangaTitle};

#[derive(Debug)]
pub struct DetectLastEpUpdated {
    pub title: MangaTitle,
    pub url: url::Url,
    pub episode: MangaEpisode,
}
