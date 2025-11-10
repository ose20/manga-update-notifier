use anyhow::{Result, anyhow};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    command::{create_manga::CreateManga, delete_manga::DeleteManga, update_manga::UpdateManga},
    manga::{Manga, portal::manga_url::MangaUrl, repository::MangaRepository},
};

use crate::repository::{
    ConnectionPool,
    manga::model::{MangaRow, portal_kind::PortalKindValue},
};

pub mod model;

#[derive(new)]
pub struct MangaRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl MangaRepository for MangaRepositoryImpl {
    async fn create_manga(&self, command: CreateManga) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO manga (title, short_title, crawl_url, public_url, portal_kind)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            command.title.inner_ref(),
            command.short_title.inner_ref(),
            command.portal.get_crawl_url().as_str(),
            command.portal.get_public_url().as_str(),
            PortalKindValue::from(command.portal.kind()).inner(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| anyhow!("{}", e))?;

        Ok(())
    }

    async fn update_manga(&self, command: UpdateManga) -> Result<()> {
        let res = sqlx::query!(
            r#"
                UPDATE manga
                SET 
                    title = $1,
                    short_title = $2,
                    crawl_url = $3,
                    public_url = $4,
                    portal_kind = $5,
                    episode = $6
                WHERE manga_id = $7
            "#,
            command.title.inner_ref(),
            command.short_title.inner_ref(),
            command.portal.get_crawl_url().as_str(),
            command.portal.get_public_url().as_str(),
            PortalKindValue::from(command.portal.kind()).inner(),
            command
                .episode
                .map_or("NULL".to_string(), |e| e.inner_ref().to_string()),
            command.manga_id.inner(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| anyhow!("{}", e))?;

        if res.rows_affected() < 1 {
            return Err(anyhow!("No manga found with the given ID"));
        }
        Ok(())
    }

    async fn delete_manga(&self, command: DeleteManga) -> Result<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM manga
                WHERE manga_id = $1
            "#,
            command.manga_id.inner(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| anyhow!("{}", e))?;

        if res.rows_affected() < 1 {
            return Err(anyhow!("No manga found with the given ID"));
        }
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Manga>> {
        let rows: Vec<MangaRow> = sqlx::query_as!(
            MangaRow,
            r#"
                SELECT manga_id, title, short_title, crawl_url, public_url, portal_kind, episode
                FROM manga
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| anyhow!("{}", e))?;

        let mangas: Vec<Manga> = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<Manga>>>()?;

        Ok(mangas)
    }
}
