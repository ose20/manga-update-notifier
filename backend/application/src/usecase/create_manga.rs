use std::sync::Arc;

use anyhow::Result;
use domain::manga::repository::{CreateManga, MangaRepository};

pub async fn execute<R>(repo: Arc<R>, command: CreateManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.create_manga(command).await
}
