use std::process;

use anyhow::Result;
use shared::config;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("error: {:#?}", e);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let app_config = config::AppConfig::new().expect("app configの取得");
    let app_registry = registry::AppRegistry::new(&app_config);

    let main_ch = discord::Channel::new(app_config.discord.main_ch_id, &app_config.discord.token);
    let err_ch = discord::Channel::new(app_config.discord.err_ch_id, &app_config.discord.token);

    let manga_result = manga::check_update(&app_registry).await;

    for update_result in manga_result.into_iter() {
        match update_result {
            Ok(Some((title, episode, url))) => {
                let _ = main_ch
                    .print(&format!(
                        "最新話が更新されました！\nタイトル: {}\nエピソード: {}\nリンク: {}",
                        title, episode, url
                    ))
                    .await?;
            }
            Ok(None) => {
                // 更新なし
            }
            Err(e) => {
                err_ch.print(&format!("err発生: {e}")).await?;
            }
        }
    }

    main_ch
        .print("本日の更新チェック作業が終了しました")
        .await?;

    Ok(())
}
