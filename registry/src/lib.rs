use std::sync::Arc;

use adapter::{database::connect_database_with, repository::manga::MangaRepositoryImpl};
use kernel::repository::manga::MangaRepository;
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    manga_repository: Arc<dyn MangaRepository>,
}

impl AppRegistry {
    pub fn new(config: &AppConfig) -> Self {
        let pool = connect_database_with(&config.database);
        let manga_repository = Arc::new(MangaRepositoryImpl::new(pool.clone()));
        Self { manga_repository }
    }

    pub fn manga_repository(&self) -> Arc<dyn MangaRepository> {
        self.manga_repository.clone()
    }
}
