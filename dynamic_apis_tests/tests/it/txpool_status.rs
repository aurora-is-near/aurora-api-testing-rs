use tracing::info;

use crate::common::init;

#[tokio::test]
async fn test_txpool_status() -> anyhow::Result<()> {
    let _guard = init();
    info!("txpool_status is not supported in Aurora");
    Ok(())
}
