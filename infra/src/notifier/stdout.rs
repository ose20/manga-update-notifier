use anyhow::Result;
use application::port::notifier::Notifier;

pub struct StdOutNotifier;

impl StdOutNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StdOutNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Notifier for StdOutNotifier {
    async fn notify_latest_episode(&self, event: domain::event::DetectLastEpUpdated) -> Result<()> {
        println!(
            "最新話が更新されました！\nタイトル: {}\nエピソード: {}\nURL: {}",
            event.title.inner_ref(),
            event.episode.inner_ref(),
            event.url
        );
        Ok(())
    }

    async fn notify_error(&self, event: domain::event::DetectFetchError) -> Result<()> {
        eprintln!(
            "最新話の取得中にエラーが発生しました。\nタイトル: {}\nエラー内容: {}",
            event.title.inner_ref(),
            event.error_message
        );
        Ok(())
    }
}
