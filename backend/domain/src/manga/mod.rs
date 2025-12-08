pub mod episode;
pub mod id;
pub mod portal;
pub mod repository;
pub mod short_title;
pub mod title;

pub use episode::MangaEpisode;
pub use id::MangaId;
pub use short_title::MangaShortTitle;
pub use title::MangaTitle;

use crate::manga::portal::MangaPortal;

#[derive(Debug, Clone)]
pub struct Manga {
    pub id: MangaId,
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub portal: MangaPortal,
    pub episode: Option<MangaEpisode>,
}

impl Manga {
    pub fn is_updated(&self, latest_ep: &MangaEpisode) -> bool {
        match &self.episode {
            Some(current_ep) => latest_ep != current_ep,
            None => true, // エピソード情報がない場合は常に更新とみなす
        }
    }

    pub fn update_episode(self, latest_ep: MangaEpisode) -> Self {
        Self {
            episode: Some(latest_ep),
            ..self
        }
    }
}
