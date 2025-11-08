use std::sync::Arc;

use anyhow::Result;
use application::port::{latest_episode_fetcher::LatestEpisodeFetcher, notifier::Notifier};
use domain::manga::repository::MangaRepository;
use infra::{
    fetcher::{LatestEpisodeFetcherImpl, webdriverpool::DriverPool},
    notifier::{discord::DiscordNotifier, stdout::StdOutNotifier},
    repository::{connect_database_with, manga::MangaRepositoryImpl},
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    book_repository: Arc<dyn MangaRepository>,
    latest_episode_fetcher: Arc<dyn LatestEpisodeFetcher>,
    notifier: Arc<dyn Notifier>,
}

impl AppRegistryImpl {
    async fn new(app_config: AppConfig) -> Result<Self> {
        let pool = connect_database_with(&app_config.database);
        let manga_repo = MangaRepositoryImpl::new(pool);
        let notifier: Arc<dyn Notifier> = if let Some(config) = app_config.discord_notifier {
            Arc::new(DiscordNotifier::new(config))
        } else {
            Arc::new(StdOutNotifier::new())
        };
        let webdriver = DriverPool::new(app_config.webdriver).await?;
        let latest_episode_fetcher = Arc::new(LatestEpisodeFetcherImpl::new(webdriver));

        Ok(Self {
            book_repository: Arc::new(manga_repo),
            latest_episode_fetcher,
            notifier,
        })
    }
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn manga_repository(&self) -> Arc<dyn MangaRepository>;
    fn latest_episode_fetcher(&self) -> Arc<dyn LatestEpisodeFetcher>;
    fn notifier(&self) -> Arc<dyn Notifier>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn manga_repository(&self) -> Arc<dyn MangaRepository> {
        self.book_repository.clone()
    }
    fn latest_episode_fetcher(&self) -> Arc<dyn LatestEpisodeFetcher> {
        self.latest_episode_fetcher.clone()
    }
    fn notifier(&self) -> Arc<dyn Notifier> {
        self.notifier.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;

pub async fn init_app_registry(app_config: AppConfig) -> Result<AppRegistry> {
    let registry = AppRegistryImpl::new(app_config).await?;
    Ok(Arc::new(registry))
}
