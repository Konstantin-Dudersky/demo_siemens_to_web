use tracing::{info, Level};
use tracing_loki::url::Url;
use tracing_subscriber::{filter::FilterFn, prelude::*};

pub async fn log_init() {
    let my_filter = FilterFn::new(|metadata| {
        let level = metadata.level();
        let module_path = metadata.module_path().unwrap_or_default();

        if module_path.starts_with("hyper::") {
            return level <= &Level::INFO;
        }
        if module_path.starts_with("opcua::") {
            return level <= &Level::WARN;
        }
        if module_path.starts_with("tokio_util::") {
            return level <= &Level::INFO;
        }

        true
    });

    let (layer_loki, task) = tracing_loki::builder()
        .label("service", "opcua-client")
        .unwrap()
        .build_url(Url::parse("http://localhost:3100").unwrap())
        .unwrap();

    tracing_subscriber::registry()
        .with(layer_loki.with_filter(my_filter))
        .init();

    tokio::spawn(task);

    info!("service started");
}
