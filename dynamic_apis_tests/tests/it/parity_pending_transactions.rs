use tracing::info;

use crate::common::init;

#[tokio::test]
async fn test_parity_pending_transactions() -> anyhow::Result<()> {
    let _guard = init();
    info!("parity_pendingTransactions is not supported in Aurora");
    Ok(())
}
