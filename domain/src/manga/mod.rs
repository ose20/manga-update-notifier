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

#[derive(Debug, Clone)]
pub struct Manga {
    pub id: MangaId,
    pub title: MangaTitle,
    pub short_title: MangaShortTitle,
    pub url: url::Url,
    pub episode: Option<MangaEpisode>,
    pub portal_kind: PortalKind,
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
