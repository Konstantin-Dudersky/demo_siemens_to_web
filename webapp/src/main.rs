mod log;

use gloo::{console, net::http::Request};
use leptos::*;

use messages;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(false);

    let start = create_action(|_| async {
        let msg = messages::Messages::CommandStart(
            messages::types::SimpleValue::new(()),
        );
        let resp = Request::put("http://localhost:3001/value/test_msg")
            .json(&msg)
            .unwrap()
            .send()
            .await
            .unwrap();
        console::log!(resp.text().await.unwrap());
    });

    let async_data = create_resource(
        move || count.get(),
        |value| async move {
            // let resp = Request::get("http://localhost:3001/value/test_msg")
            //     .send()
            //     .await
            //     .unwrap();
            // console::log!(resp.text().await.unwrap());
        },
    );

    view! {
        <button on:click=move |_| {
            start.dispatch(());
        }>
            "Start"
        </button>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App/> })
}
