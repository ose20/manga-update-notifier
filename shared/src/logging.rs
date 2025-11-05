use anyhow::Result;
use tracing_subscriber::{
    EnvFilter, fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init_logger() -> Result<()> {
    let fileter = EnvFilter::new("info");
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(ChronoLocal::rfc_3339());

    tracing_subscriber::registry()
        .with(fileter)
        .with(subscriber)
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;

    Ok(())
}
