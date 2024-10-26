use anyhow::{Ok, Result};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::manga::{CreateManga, DeleteManga, Manga, UpdateMangaPersistence},
    repository::manga::MangaRepository,
};

use crate::database::{model::manga::MangaRow, ConnectionPool};

#[derive(new)]
pub struct MangaRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl MangaRepository for MangaRepositoryImpl {
    async fn create(&self, event: CreateManga) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO mangas (title, short_title, episode, url)
                VALUES($1, $2, $3, $4)
            "#,
            event.title,
            event.short_title,
            event.episode,
            event.url,
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn delete(&self, event: DeleteManga) -> Result<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM mangas WHERE short_title = $1
            "#,
            event.short_title
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(result.rows_affected())
    }

    async fn find_by_short_title(&self, short_title: &str) -> Result<Option<Manga>> {
        let row: Option<MangaRow> = sqlx::query_as!(
            MangaRow,
            r#"
                SELECT
                    manga_id,
                    title,
                    short_title,
                    episode,
                    url
                FROM mangas
                WHERE short_title = $1
            "#,
            short_title
        )
        .fetch_optional(self.db.inner_ref())
        .await?;

        Ok(row.map(Manga::from))
    }

    async fn update_episode_by_short_title(&self, short_title: &str, episode: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE mangas SET episode = $1 WHERE short_title = $2",
            episode,
            short_title,
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn update_persistence(&self, event: UpdateMangaPersistence) -> Result<u64> {
        let result = sqlx::query!(
            "
                INSERT INTO mangas (title, url, short_title)
                VALUES ($1, $2, $3)
                ON CONFLICT (short_title)
                DO UPDATE SET title = EXCLUDED.title, url = EXCLUDED.url
            ",
            event.title,
            event.url,
            event.short_title
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(result.rows_affected())
    }
}
