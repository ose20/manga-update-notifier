use anyhow::Result;
use application::port::latest_episode_fetcher::{FetchLatestEpCommand, LatestEpisodeFetcher};
use domain::manga::{MangaEpisode, portal::portal_kind::PortalKind};
use serenity::async_trait;

use crate::fetcher::webdriverpool::DriverPool;

pub mod portal_kind;
pub mod webdriverpool;

pub struct LatestEpisodeFetcherImpl {
    webdriver_pool: DriverPool,
}

impl LatestEpisodeFetcherImpl {
    pub fn new(webdriverpool: DriverPool) -> Self {
        Self {
            webdriver_pool: webdriverpool,
        }
    }
}

#[async_trait]
impl LatestEpisodeFetcher for LatestEpisodeFetcherImpl {
    async fn fetch_latest_episode(&self, command: FetchLatestEpCommand) -> Result<MangaEpisode> {
        // portal_kind ごとに異なるロジックを呼び出す
        match command.portal_kind {
            PortalKind::ComicDays => {
                let crawler = portal_kind::comic_days::ComicDaysCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::ComicFuz => {
                let crawler = portal_kind::comic_fuz::ComicFuzCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::KimiComi => {
                let crawler = portal_kind::kimi_comi::KimiComiEpCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::KadoComi => {
                let crawler = portal_kind::kado_comi::KadoComiEpCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::YoungMagazine => {
                let crawler = portal_kind::young_magazine::YoungMagazineEpCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::HerosWeb => {
                let crawler = portal_kind::heros_web::HerosWebCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::TonarinoYJ => {
                let crawler = portal_kind::tonarino_yj::TonarinoYJCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::WebAce => {
                let crawler = portal_kind::web_ace::WebAceEpCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::JumpPlus => {
                let crawler = portal_kind::jump_plus::JumpPlusCrawler::new(command);
                self.webdriver_pool.with_driver(crawler).await
            }
            PortalKind::ComicZenon => {
                let crawler = portal_kind::comic_zenon::ComicDaysCrawler::new(command);
                crawler.crawl().await
            }
        }
    }
}
