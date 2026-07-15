pub fn init() {
    let _ignored = tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .try_init();
}
