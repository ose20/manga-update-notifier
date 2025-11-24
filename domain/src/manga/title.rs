use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaTitle(String);

impl MangaTitle {
    pub fn new(title: String) -> Result<Self> {
        if title.trim().is_empty() {
            anyhow::bail!("Manga title cannot be empty");
        }
        Ok(MangaTitle(title))
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
