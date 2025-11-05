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
    todo!()
}
