#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaTitle(String);

impl MangaTitle {
    pub fn new(title: String) -> Self {
        MangaTitle(title)
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
