use tracing::subscriber::DefaultGuard;

pub fn init() -> DefaultGuard {
    // Build a subscriber with the non-blocking file appender
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // .with_writer(non_blocking)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_default(subscriber)
}
