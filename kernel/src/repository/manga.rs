use anyhow::Result;
use async_trait::async_trait;

use crate::model::manga::{CreateManga, DeleteManga, Manga, UpdateMangaPersistence};

#[async_trait]
pub trait MangaRepository: Send + Sync {
    async fn create(&self, event: CreateManga) -> Result<()>;
    async fn find_by_short_title(&self, short_title: &str) -> Result<Option<Manga>>;
    async fn update_episode_by_short_title(&self, short_title: &str, episode: &str) -> Result<()>;
    async fn update_persistence(&self, event: UpdateMangaPersistence) -> Result<u64>;
    async fn delete(&self, event: DeleteManga) -> Result<u64>;
}
