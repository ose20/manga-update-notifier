use anyhow::Result;
use domain::{event::latest_ep_updated::DetectLastEpUpdated, manga::Manga};
use registry::AppRegistry;

pub async fn execute(_registry: AppRegistry, _manga: Manga) -> Result<Option<DetectLastEpUpdated>> {
    todo!()
}
