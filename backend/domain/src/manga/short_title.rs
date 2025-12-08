use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaShortTitle(String);

impl MangaShortTitle {
    pub fn new(short_title: String) -> Result<Self> {
        if short_title.trim().is_empty() {
            anyhow::bail!("Manga short title cannot be empty");
        }
        Ok(MangaShortTitle(short_title))
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
