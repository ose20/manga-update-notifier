use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaEpisode(String);

impl MangaEpisode {
    pub fn new(episode: String) -> Result<Self> {
        if episode.trim().is_empty() {
            anyhow::bail!("Manga episode cannot be empty");
        }
        Ok(MangaEpisode(episode))
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
