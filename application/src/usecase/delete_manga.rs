use std::sync::Arc;

use anyhow::Result;
use domain::{command, manga::repository::MangaRepository};

pub async fn execute<R>(repo: Arc<R>, command: command::DeleteManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.delete_manga(command).await
}
