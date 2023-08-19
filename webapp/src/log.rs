pub use tracing::Level;
use tracing_subscriber;

pub fn init() {
    let event_format = tracing_subscriber::fmt::format().pretty();
    let file_writer = tracing_appender::rolling::hourly("logs", "log");

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .event_format(event_format)
        .with_writer(file_writer)
        .init();
}
