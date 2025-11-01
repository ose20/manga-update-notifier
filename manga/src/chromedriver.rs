use std::time::Duration;

use anyhow::{anyhow, Ok, Result};
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};

pub async fn get_driver() -> Result<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless=new")?;
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    caps.add_arg(&format!("--user-agent={}", user_agent))?;
    caps.add_arg("--lang=ja")?; // 言語設定を日本語に

    // あとで環境変数に変えたい
    // driverConfigあたりをsharedで作る？
    // caps.set_binary("/home/ose20/app/chrome-linux64/chrome")?;

    let webdriver = WebDriver::new("http://localhost:4444", caps)
        .await
        .map_err(|e| anyhow!("{}", e))?;
    webdriver
        .set_page_load_timeout(Duration::from_secs(300))
        .await?;

    Ok(webdriver)
}
