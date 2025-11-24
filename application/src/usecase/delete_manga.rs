use std::sync::Arc;

use anyhow::Result;
use domain::manga::repository::{DeleteManga, MangaRepository};

pub async fn execute<R>(repo: Arc<R>, command: DeleteManga) -> Result<()>
where
    R: MangaRepository + ?Sized,
{
    repo.delete_manga(command).await
}
