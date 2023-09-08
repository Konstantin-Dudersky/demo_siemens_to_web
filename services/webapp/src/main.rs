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

    let command_start = create_action(|_: &()| async {
        let msg = messages::Messages::CommandStart(
            messages::types::Command::new(None),
        );
        send_message_to_api(msg).await;
    });

    let command_stop = create_action(|_: &()| async {
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
        <div class="container mx-auto">
            <div class="flex flex-row">
                <div class="basis-1/2">
                    <p class="m-4">
                        Состояние
                    </p>
                </div>
                <div class="basis-1/2">
                    <p class="m-4">
                    <State res=motor_state/>
                    // {move || motor_state.get()}
                    </p>
                </div>

            </div>
            <div class="flex flex-row">
                <div class="basis-1/2">
                    <Button
                    label="Start".to_string()
                    action=command_start
                    />
                </div>
                <div class="basis-1/2">
                    <Button
                    label="Stop".to_string()
                    action=command_stop
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Button(label: String, action: Action<(), ()>) -> impl IntoView {
    view! {
        <div
            on:click=move |_| { action.dispatch(()) }
            class="pointer-events-auto rounded-md bg-indigo-600 py-2 px-3 text-[0.8125rem] font-semibold leading-5 text-white hover:bg-indigo-500 m-4"
            >
            { label }
        </div>
    }
}

#[component]
fn State(res: Resource<bool, i16>) -> impl IntoView {
    let text = move || match res.get() {
        Some(0) => "Стоп",
        Some(1) => "Работа",
        _ => "Неизвестно",
    };

    view! {
        <span class="inline-flex items-center rounded-md px-2 py-1 text-sm font-semibold ring-1 ring-inset ring-gray-500/10"

        class=("bg-grey-50", move || {res.get() == Some(0)})
        class=("text-gray-600", move || {res.get() == Some(0)})

        class=("bg-green-50", move || {res.get() == Some(1)})
        class=("text-green-700", move || {res.get() == Some(1)})
        >
            { text }
        </span>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App/> })
}