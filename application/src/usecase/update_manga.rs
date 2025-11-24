use std::sync::Arc;

use anyhow::Result;
use domain::manga::repository::{MangaRepository, UpdateManga};

pub async fn execute<R>(repo: Arc<R>, command: UpdateManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.update_manga(command).await
}
