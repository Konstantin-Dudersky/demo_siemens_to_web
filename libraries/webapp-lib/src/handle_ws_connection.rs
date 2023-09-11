use futures_util::StreamExt;
use gloo::{
    console,
    net::websocket::{futures::WebSocket, Message},
};

pub async fn handle_ws_connection(ws_url: &str, process_msg: fn(&str) -> ()) {
    let ws = WebSocket::open(&ws_url).unwrap();
    let (_, mut read) = ws.split();
    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            let msg = match msg {
                Message::Text(value) => value,
                Message::Bytes(_) => "123".to_string(),
            };
            process_msg(&msg);
        };
    }
    console::log!("WebSocket Closed")
}
