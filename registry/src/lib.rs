use std::sync::Arc;

use domain::manga::repository::MangaRepository;
use infra::repository::{connect_database_with, manga::MangaRepositoryImpl};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    book_repository: Arc<dyn MangaRepository>,
}

impl AppRegistryImpl {
    pub fn new(app_config: AppConfig) -> Self {
        let pool = connect_database_with(&app_config.database);
        let manga_repo = MangaRepositoryImpl::new(pool);

        Self {
            book_repository: Arc::new(manga_repo),
        }
    }
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn manga_repository(&self) -> Arc<dyn MangaRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn manga_repository(&self) -> Arc<dyn MangaRepository> {
        self.book_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
