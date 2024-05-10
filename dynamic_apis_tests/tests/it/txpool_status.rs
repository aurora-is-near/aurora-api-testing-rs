use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::test]
async fn test_txpool_status() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    info!("txpool_status is not supported in Aurora");
    Ok(())
}
