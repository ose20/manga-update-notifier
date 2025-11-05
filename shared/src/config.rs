use anyhow::Result;

pub struct AppConfig {
    pub database: DatabaseConfig,
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
        Ok(AppConfig { database })
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
