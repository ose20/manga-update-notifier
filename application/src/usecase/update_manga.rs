use std::sync::Arc;

use anyhow::Result;
use domain::{command, manga::repository::MangaRepository};

pub async fn execute<R>(repo: Arc<R>, command: command::UpdateManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.update_manga(command).await
}
