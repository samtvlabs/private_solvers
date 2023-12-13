use anyhow::Result;
use khalani_solver::diagnostics::logs::configure_logs;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Cross-Chain Market Maker");
    Ok(())
}
