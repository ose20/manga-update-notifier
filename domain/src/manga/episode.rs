#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaEpisode(String);

impl MangaEpisode {
    pub fn new(episode: String) -> Self {
        assert!(!episode.is_empty(), "Episode cannot be empty");
        MangaEpisode(episode)
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
