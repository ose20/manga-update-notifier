use anyhow::Result;
use async_trait::async_trait;

use crate::{
    command::{create_manga::CreateManga, delete_manga::DeleteManga, update_manga::UpdateManga},
    manga::Manga,
};

#[async_trait]
pub trait MangaRepository: Send + Sync {
    async fn create_manga(&self, command: CreateManga) -> Result<()>;
    async fn update_manga(&self, command: UpdateManga) -> Result<()>;
    async fn delete_manga(&self, command: DeleteManga) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<Manga>>;
}
