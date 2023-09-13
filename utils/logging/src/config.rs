use tracing::{info, Level};
use tracing_loki::url::Url;
use tracing_subscriber::{filter::FilterFn, prelude::*};

use crate::errors::Errors;

pub async fn configure_logging(
    service: &str,
    loki_url: &str,
    min_level: Level,
) -> Result<(), Errors> {
    let min_level_clone = min_level.clone();
    let my_filter = FilterFn::new(move |metadata| {
        let level = *metadata.level();
        let module_path = metadata.module_path().unwrap_or_default();

        if module_path.starts_with("hyper::") {
            return level <= Level::INFO;
        }
        if module_path.starts_with("opcua::") {
            return level <= Level::WARN;
        }
        if module_path.starts_with("tokio_util::") {
            return level <= Level::INFO;
        }
        if module_path.starts_with("sqlx::query::") {
            return level <= Level::INFO;
        }
        if module_path.starts_with("tokio_tungstenite") {
            return level <= Level::INFO;
        }
        if module_path.starts_with("tungstenite") {
            return level <= Level::INFO;
        }

        if level > min_level_clone {
            return false;
        }

        true
    });

    let (layer_loki, task) = tracing_loki::builder()
        .label("service", service)?
        .build_url(Url::parse(loki_url)?)?;

    let layer_stdout = tracing_subscriber::fmt::Layer::new().pretty();

    tracing_subscriber::registry()
        .with(layer_loki.with_filter(my_filter.clone()))
        .with(layer_stdout.with_filter(my_filter.clone()))
        .init();

    tokio::spawn(task);

    info!("service {} started", service);
    Ok(())
}
