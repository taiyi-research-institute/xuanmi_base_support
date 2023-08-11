pub use tracing;

pub fn init_tracer(log_dir: &str, log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    let file_appender = tracing_appender::rolling::daily(log_dir, "lbclient.log");
    let (nbl, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(nbl)
        .with_max_level(level)
        .with_thread_ids(true)
        // without unsetting ANSI mode, the log file will be flattered with ANSI escape codes.
        .with_ansi(false)
        .compact()
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
