use anyhow::Result;
use thirtyfour::{ChromeCapabilities, ChromiumLikeCapabilities, DesiredCapabilities};
use tracing::debug;

#[derive(Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub discord_notifier: Option<DiscordNotifierConfig>,
    pub webdriver: WebdriverConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
            port: std::env::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set")
                .parse()
                .expect("DATABASE_PORT must be a number"),
            username: std::env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set"),
            password: std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            database: std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
        };

        debug!("DatabaseConfig loaded: {:?}", database);

        let discord_notifier = if let Ok(val) = std::env::var("NOTIFY_DISCORD") {
            if val == "TRUE" {
                dotenv::from_filename("secret.env").ok();
                let channel_id = std::env::var("DISCORD_CHANNEL_ID")
                    .expect("DISCORD_CHANNEL_ID must be set")
                    .parse()
                    .expect("DISCORD_CHANNEL_ID must be a number");
                let err_channel_id = std::env::var("DISCORD_ERR_CHANNEL_ID")
                    .expect("DISCORD_ERR_CHANNEL_ID must be set")
                    .parse()
                    .expect("DISCORD_ERR_CHANNEL_ID must be a number");
                let token =
                    std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set");
                Some(DiscordNotifierConfig {
                    channel_id,
                    err_channel_id,
                    token,
                })
            } else {
                None
            }
        } else {
            None
        };
        let webdriver = WebdriverConfig::new()?;
        Ok(AppConfig {
            database,
            discord_notifier,
            webdriver,
        })
    }
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug)]
pub struct DiscordNotifierConfig {
    pub channel_id: u64,
    pub err_channel_id: u64,
    pub token: String,
}

const WEB_DRIVER_ENDPOINTS: [&str; 4] = [
    "http://localhost:4444",
    "http://localhost:4445",
    "http://localhost:4446",
    "http://localhost:4447",
];

#[derive(Debug)]
pub struct WebdriverConfig {
    pub endpoints: Vec<String>,
    pub chrome_caps: ChromeCapabilities,
}

impl WebdriverConfig {
    pub fn new() -> Result<Self> {
        let endpoints = WEB_DRIVER_ENDPOINTS.iter().map(|s| s.to_string()).collect();
        let mut caps = DesiredCapabilities::chrome();
        caps.add_arg("--headless=new")?;
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        caps.add_arg(&format!("--user-agent={}", user_agent))?;
        caps.add_arg("--no-sandbox")?;
        caps.add_arg("--disable-dev-shm-usage")?;
        Ok(WebdriverConfig {
            endpoints,
            chrome_caps: caps,
        })
    }
}
