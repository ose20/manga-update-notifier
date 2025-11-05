use anyhow::Result;
use domain::{event::last_ep_detected::DetectLastEpUpdatedEvent, manga::Manga};
use registry::AppRegistry;

pub async fn execute(
    _registry: AppRegistry,
    _manga: Manga,
) -> Result<Option<DetectLastEpUpdatedEvent>> {
    todo!()
}
