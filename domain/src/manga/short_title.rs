#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MangaShortTitle(String);

impl MangaShortTitle {
    pub fn new(short_title: String) -> Self {
        MangaShortTitle(short_title)
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
