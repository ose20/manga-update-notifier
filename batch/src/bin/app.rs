use anyhow::Result;
use shared::logging::init_logger;
use tracing::error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    init_logger()?;

    // registry を作る

    // manga 一覧を取得する
    // 各 manga について、以下の非同期タスクを作る
    // check_and_notify を実行する
    // これを全部待つ(どれかが失敗しても待つ)
    // 失敗したやつはエラーメッセージだけ通知して終わる

    todo!()
}
