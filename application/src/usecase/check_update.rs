use std::sync::Arc;

use domain::{
    event::{DetectFetchError, latest_ep_updated::DetectLastEpUpdated},
    manga::Manga,
};

use crate::port::latest_episode_fetcher::LatestEpisodeFetcher;

pub async fn execute<F>(
    fetcher: Arc<F>,
    manga: Manga,
) -> Result<Option<(DetectLastEpUpdated, Manga)>, DetectFetchError>
where
    F: LatestEpisodeFetcher + ?Sized,
{
    let latest_ep = fetcher
        .fetch_latest_episode(manga.clone().into())
        .await
        .map_err(|e| {
            DetectFetchError::new(manga.title.clone(), manga.url.clone(), e.to_string())
        })?;

    if manga.is_updated(&latest_ep) {
        let update_manga = manga.update_episode(latest_ep.clone());
        Ok(Some((
            DetectLastEpUpdated {
                title: update_manga.title.clone(),
                url: update_manga.url.clone(),
                episode: latest_ep,
            },
            update_manga,
        )))
    } else {
        Ok(None)
    }
}
