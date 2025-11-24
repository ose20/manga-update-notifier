use anyhow::{Result, anyhow};
use async_trait::async_trait;
use derive_new::new;
use domain::manga::{
    Manga,
    portal::manga_url::MangaUrl,
    repository::{CreateManga, DeleteManga, MangaRepository, UpdateManga},
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

#[cfg(test)]
mod tests {

    use super::*;
    use domain::manga::portal::{MangaPortal, portal_kind::PortalKind};

    #[sqlx::test]
    async fn test_insert_manga(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = MangaRepositoryImpl::new(ConnectionPool::new(pool.clone()));

        let crawl_url = url::Url::parse("https://example.com/crawl")?;
        let public_url = crawl_url.clone();
        let portal_kind = PortalKind::KadoComi;
        let portal = MangaPortal::new(portal_kind.clone(), crawl_url.clone(), public_url.clone())?;
        let create_command = CreateManga {
            title: domain::manga::MangaTitle::new("Test Manga".to_string())?,
            short_title: domain::manga::MangaShortTitle::new("Test".to_string())?,
            portal,
        };

        repo.create_manga(create_command).await?;
        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);
        let manga = &res[0];
        assert_eq!(manga.title.inner_ref(), "Test Manga");
        assert_eq!(manga.short_title.inner_ref(), "Test");
        assert_eq!(manga.portal.get_crawl_url(), &crawl_url);
        assert_eq!(manga.portal.get_public_url(), &public_url);
        assert_eq!(manga.portal.kind(), portal_kind);
        Ok(())
    }

    #[sqlx::test(fixtures("manga"))]
    async fn test_find_all(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = MangaRepositoryImpl::new(ConnectionPool::new(pool.clone()));

        let mangas = repo.find_all().await?;
        assert_eq!(mangas.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures("manga"))]
    async fn test_delete_manga(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = MangaRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let mangas = repo.find_all().await?;

        let delete_command = DeleteManga {
            manga_id: mangas[0].id.clone(),
        };

        repo.delete_manga(delete_command).await?;
        let mangas_after_delete = repo.find_all().await?;
        assert_eq!(mangas_after_delete.len(), mangas.len() - 1);

        Ok(())
    }

    #[sqlx::test(fixtures("manga"))]
    async fn test_update_manga(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = MangaRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let mangas = repo.find_all().await?;

        let updated_title = "Updated Title".to_string();
        let updated_short_title = "UpdatedShort".to_string();
        let crawsl_url = url::Url::parse("https://example.com/updated_crawl")?;
        let public_url = url::Url::parse("https://example.com/updated_public")?;
        let portal_kind = PortalKind::WebAce;
        let portal = MangaPortal::new(portal_kind.clone(), crawsl_url.clone(), public_url.clone())?;
        let updated_episode = "Episode 2".to_string();

        let update_command = UpdateManga {
            manga_id: mangas[0].id.clone(),
            title: domain::manga::MangaTitle::new(updated_title.clone())?,
            short_title: domain::manga::MangaShortTitle::new(updated_short_title.clone())?,
            portal,
            episode: Some(domain::manga::episode::MangaEpisode::new(
                updated_episode.clone(),
            )?),
        };

        repo.update_manga(update_command).await?;
        let mangas_after_update = repo.find_all().await?;
        let updated_manga = mangas_after_update
            .into_iter()
            .find(|m| m.id == mangas[0].id)
            .unwrap();

        assert_eq!(updated_manga.title.inner_ref(), updated_title);
        assert_eq!(updated_manga.short_title.inner_ref(), updated_short_title);
        assert_eq!(updated_manga.portal.get_crawl_url(), &crawsl_url);
        assert_eq!(updated_manga.portal.get_public_url(), &public_url);
        assert_eq!(updated_manga.portal.kind(), portal_kind);
        assert_eq!(updated_manga.episode.unwrap().inner_ref(), updated_episode);

        Ok(())
    }
}
