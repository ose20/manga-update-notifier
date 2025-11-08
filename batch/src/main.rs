use anyhow::Result;
use application::usecase;
use futures::future;
use shared::{config, logging::init_logger};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    init_logger()?;
    let app_config = config::AppConfig::new()?;
    let app_registry = registry::init_app_registry(app_config).await?;
    info!("App registry initialized");

    let mangas = app_registry.manga_repository().find_all().await?;
    info!("Found {} mangas to check for updates", mangas.len());
    // 各 manga について、以下の非同期タスクを作る
    let tasks = mangas.into_iter().map(|manga| {
        let fetcher = app_registry.latest_episode_fetcher();
        let notifier = app_registry.notifier();
        let app_registry = app_registry.clone();
        async move {
            match usecase::check_update::execute(fetcher, manga).await {
                Ok(Some((event, updated_manga))) => {
                    // Todo: 以下の二つの処理はトランザクションにまとめたい
                    // せっかくドメインイベントを使っているので、このイベントをフックにトランザクションを実行するワークフローを組み立てたい
                    // そうすると、event の中に updated_manga が含まれていても良さそう
                    notifier.notify_latest_episode(event).await?;
                    usecase::update_manga::execute(
                        app_registry.clone().manga_repository(),
                        updated_manga.into(),
                    )
                    .await
                }
                Ok(None) => Ok(()),
                Err(event) => notifier.notify_error(event).await,
            }
        }
    });

    info!("Starting check and notify tasks for all mangas");
    // これを全部並行実行して結果を待つ(どれかが失敗しても待つ)
    let results: Vec<Result<()>> = future::join_all(tasks).await;

    // 失敗した task はログに出す
    for result in results {
        if let Err(err) = result {
            error!("Error in check and notify task: {}", err);
        }
    }

    Ok(())
}
