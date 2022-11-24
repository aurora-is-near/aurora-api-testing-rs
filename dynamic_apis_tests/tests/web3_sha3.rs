use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::test]
async fn test_web3_sha3() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _t = tracing::subscriber::set_global_default(subscriber);
    info!("web3_sha3 is not supported in Aurora");
    Ok(())
}
