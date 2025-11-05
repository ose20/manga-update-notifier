use crate::manga::{MangaEpisode, MangaTitle};

#[derive(Debug)]
pub struct DetectLastEpUpdatedEvent {
    pub title: MangaTitle,
    pub url: url::Url,
    pub episode: MangaEpisode,
}
