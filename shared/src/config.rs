use anyhow::Result;

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };

        dotenv::from_filename("secret.env").ok();
        let discord = DiscordConfig {
            token: dotenv::var("TOKEN")?,
            main_ch_id: dotenv::var("MAIN_CH_ID")?.parse()?,
            err_ch_id: dotenv::var("ERR_CH_ID")?.parse()?,
        };

        Ok(Self { database, discord })
    }

    // テスト用
    pub fn new_dev() -> Result<Self> {
        let database = DatabaseConfig {
            host: "localhost".into(),
            port: 5434,
            username: "app".into(),
            password: "passwd".into(),
            database: "app".into(),
        };

        dotenv::from_filename("secret.env").ok();
        let discord = DiscordConfig {
            token: dotenv::var("TOKEN")?,
            main_ch_id: dotenv::var("MAIN_CH_ID")?.parse()?,
            err_ch_id: dotenv::var("ERR_CH_ID")?.parse()?,
        };

        Ok(Self { database, discord })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct DiscordConfig {
    pub token: String,
    pub main_ch_id: u64,
    pub err_ch_id: u64,
}
