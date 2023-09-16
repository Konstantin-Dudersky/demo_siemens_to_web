use std::time::Duration;

use tokio::{main, time::sleep};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn, Level};

use logging::configure_logging;
use messages::Messages;
use websocket_lib::{tasks, Errors};

#[main]
async fn main() {
    let config = env_vars::load().expect("Settings not loaded");

    configure_logging("api-ws", config.loki_url.as_str(), Level::DEBUG)
        .await
        .expect("Error in logger initialization");

    loop {
        let cancel = CancellationToken::new();
        let result = tasks::task_main::<Messages>(
            config.redis_url.clone(),
            config.redis_channel.clone(),
            config.api_ws_port,
            cancel.clone(),
        )
        .await;
        match result {
            Ok(_) => (),
            Err(error) => match error {
                Errors::BindToPortError(error) => {
                    error!("Stop program: {:?}", error);
                    return;
                }
                _ => {
                    warn!("{:?}", error);
                }
            },
        };
        cancel.cancel();
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...");
    }
}
