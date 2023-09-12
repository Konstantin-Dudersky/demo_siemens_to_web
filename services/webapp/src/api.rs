use gloo::{console, net::http::Request};

use messages::Messages;

// http://localhost:3001/value/

pub async fn send_message_to_api(api_url: &str, msg: Messages) -> () {
    let url = format!("{}{}", api_url, msg.key());
    let resp = Request::put(&url).json(&msg).unwrap().send().await.unwrap();
    console::log!(resp.text().await.unwrap());
}

pub async fn get_message_from_api(key: &str) -> Messages {
    let url = format!("http://localhost:3001/value/{}", key);
    let resp = Request::get(&url).send().await.unwrap();
    let str = resp.text().await.unwrap();
    Messages::deserialize(&str).unwrap()
}
