use tracing::info;

use crate::common::init;

#[tokio::test]
async fn test_txpool_content() -> anyhow::Result<()> {
    let _guard = init();
    info!("txpool_content is not supported in Aurora");
    Ok(())
}
