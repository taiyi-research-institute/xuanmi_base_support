// 2023-08-14 14:19 (GMT+8)
// I don't exactly know why, but if one implement init_tracer as a function,
// the tracer will not be initialized. This phenomenon is repeatable.
// I suppose that the global tracer is out-of-scope when the function returns.
#[macro_export]
macro_rules! init_tracer {
    ($log_dir:expr, $logfile_prefix:expr, $log_level:expr) => {
        let level = match ($log_level).to_lowercase().as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        };
        let file_appender = xuanmi_base_support::tracing_appender::rolling::daily($log_dir, $logfile_prefix);
        let (nbl, _guard) = xuanmi_base_support::tracing_appender::non_blocking(file_appender);
        let subscriber = xuanmi_base_support::tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(nbl)
            .with_max_level(level)
            .with_thread_ids(true)
            // without unsetting ANSI mode, the log file will be flattered with ANSI escape codes.
            .with_ansi(false)
            .compact()
            .pretty()
            .finish();
        xuanmi_base_support::tracing::subscriber::set_global_default(subscriber).unwrap();
    };
}