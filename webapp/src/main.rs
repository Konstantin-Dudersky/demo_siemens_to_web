mod log;

use gloo::{console, net::http::Request};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(false);

    let async_data = create_resource(
        move || count.get(),
        |value| async move {
            let resp = Request::get("http://localhost:3001/value/test_msg")
                .send()
                .await
                .unwrap();
            console::log!(resp.text().await.unwrap());
        },
    );

    view! {
        <button on:click=move |_| {
            set_count.set(!count.get());
        }>
            "Start"
        </button>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App/> })
}
