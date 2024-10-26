use kernel::model::manga::Manga;

pub struct MangaRow {
    pub manga_id: i64,
    pub title: String,
    pub short_title: String,
    pub episode: String,
    pub url: String,
}

impl From<MangaRow> for Manga {
    fn from(value: MangaRow) -> Self {
        let MangaRow {
            manga_id,
            title,
            short_title,
            episode,
            url,
        } = value;

        Self {
            id: manga_id,
            title,
            short_title,
            episode,
            url,
        }
    }
}
