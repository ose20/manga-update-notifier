use anyhow::Result;
use application::port::notifier::Notifier;
use domain::event::{DetectFetchError, DetectLastEpUpdated};
use serenity::{
    all::{ChannelId, Http},
    async_trait,
};
use shared::config::DiscordNotifierConfig;

pub struct DiscordNotifier {
    channel_id: ChannelId,
    err_channel_id: ChannelId,
    http_client: Http,
}

impl DiscordNotifier {
    pub fn new(config: DiscordNotifierConfig) -> Self {
        let channel_id = ChannelId::new(config.channel_id);
        let err_channel_id = ChannelId::new(config.err_channel_id);
        let http_client = Http::new(&config.token);
        Self {
            channel_id,
            err_channel_id,
            http_client,
        }
    }
}

#[async_trait]
impl Notifier for DiscordNotifier {
    async fn notify_latest_episode(&self, event: DetectLastEpUpdated) -> Result<()> {
        self.channel_id
            .say(
                &self.http_client,
                format!(
                    "漫画が更新されました！\n**{}**: {}\n{}",
                    event.title.inner_ref(),
                    event.episode.inner_ref(),
                    event.url
                ),
            )
            .await?;

        Ok(())
    }

    async fn notify_error(&self, event: DetectFetchError) -> Result<()> {
        self.err_channel_id
            .say(
                &self.http_client,
                format!(
                    "最新話の取得中にエラーが発生しました。\n**{}**\nエラー内容: {}",
                    event.title.inner_ref(),
                    event.error_message
                ),
            )
            .await?;
        Ok(())
    }
}
