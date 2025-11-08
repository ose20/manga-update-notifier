use std::sync::Arc;

use anyhow::Result;
use domain::{command, manga::repository::MangaRepository};

pub async fn execute<R>(repo: Arc<R>, command: command::CreateManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.create_manga(command).await
}
