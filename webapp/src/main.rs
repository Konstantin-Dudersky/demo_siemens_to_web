mod log;

use std::time::Duration;

use gloo::{console, net::http::Request};
use leptos::*;
use serde_json::from_str as deserialize;

use messages;

async fn send_message_to_api(msg: messages::Messages) -> () {
    let url = format!("http://localhost:3001/value/{}", msg.key());
    let resp = Request::put(&url).json(&msg).unwrap().send().await.unwrap();
    console::log!(resp.text().await.unwrap());
}

async fn get_message_from_api(key: &str) -> messages::Messages {
    let url = format!("http://localhost:3001/value/{}", key);
    let resp = Request::get(&url).send().await.unwrap();
    let str = resp.text().await.unwrap();
    deserialize::<messages::Messages>(&str).unwrap()
}

#[component]
fn App() -> impl IntoView {
    let (update, set_update) = create_signal(false);

    set_interval(
        move || set_update.update(|value| *value = !*value),
        Duration::from_secs(1),
    );

    let command_start = create_action(|_| async {
        let msg = messages::Messages::CommandStart(
            messages::types::Command::new(None),
        );
        send_message_to_api(msg).await;
    });

    let command_stop = create_action(|_| async {
        let msg = messages::Messages::CommandStop(
            messages::types::Command::new(None),
        );
        send_message_to_api(msg).await;
    });

    let motor_state = create_resource(
        move || update.get(),
        |_| async move {
            let ans = get_message_from_api("MotorState").await;
            if let messages::Messages::MotorState(
                messages::types::SingleValue { value, .. },
            ) = ans
            {
                value
            } else {
                0
            }
        },
    );

    view! {
        Состояние: {move || motor_state.get()}
        <button on:click=move |_| {
            command_start.dispatch(());
        }>
            "Start"
        </button>
        <button on:click=move |_| {
            command_stop.dispatch(());
        }>
            "Stop"
        </button>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App/> })
}
