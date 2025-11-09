use anyhow::Result;
use domain::manga::MangaEpisode;
use thirtyfour::WebDriver;

pub mod comic_days;
pub mod comic_fuz;
pub mod comic_zenon;
pub mod heros_web;
pub mod jump_plus;
pub mod kado_comi;
pub mod kimi_comi;
pub mod tonarino_yj;
pub mod web_ace;
pub mod young_magazine;

#[async_trait::async_trait]
pub trait EpCrawler {
    async fn crawl(&self, deiver: &WebDriver) -> Result<MangaEpisode>;
}
