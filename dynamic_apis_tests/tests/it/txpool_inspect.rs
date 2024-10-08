use tracing::info;

use crate::common::init;

#[tokio::test]
async fn test_txpool_inspect() -> anyhow::Result<()> {
    let _guard = init();
    info!("txpool_inspect is not supported in Aurora");
    Ok(())
}
