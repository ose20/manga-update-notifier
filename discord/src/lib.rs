use anyhow::{anyhow, Result};
use serenity::all::{ChannelId, Http, Message};

pub struct Channel {
    channel_id: ChannelId,
    http_client: Http,
}

impl Channel {
    pub fn new(channel_id: u64, token: &str) -> Self {
        let channel_id = ChannelId::new(channel_id);
        let http_client = Http::new(token);

        Self {
            channel_id,
            http_client,
        }
    }

    pub async fn print(&self, msg: &str) -> Result<Message> {
        self.channel_id
            .say(&self.http_client, msg)
            .await
            .map_err(|e| anyhow!("{}", e))
    }
}
