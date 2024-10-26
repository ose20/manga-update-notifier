use anyhow::{anyhow, Result};
use registry::AppRegistry;

pub async fn extract_manga_info(
    app_registry: &AppRegistry,
    short_title: &str,
) -> Result<(String, String, String)> {
    let manga = app_registry
        .manga_repository()
        .find_by_short_title(short_title)
        .await
        .map_err(|e| anyhow!("instance 作成失敗: title: {}, err: {}", short_title, e))?
        .ok_or(anyhow!("DB にデータがありません: title: {}", short_title))?;
    Ok((manga.title, short_title.into(), manga.url))
}
