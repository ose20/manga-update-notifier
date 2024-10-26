#[derive(Debug)]
pub struct Manga {
    pub id: i64,
    pub title: String,
    pub short_title: String,
    pub episode: String,
    pub url: String, // url crate 使ってもいいかも
}

pub struct CreateManga {
    pub title: String,
    pub short_title: String,
    pub episode: String,
    pub url: String,
}

pub struct DeleteManga {
    pub short_title: String,
}

pub struct UpdateMangaPersistence {
    pub title: String,
    pub short_title: String,
    pub url: String,
}
