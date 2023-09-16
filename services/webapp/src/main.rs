use leptos::*;
use leptos_router::*;

use messages::{self, types, Messages};
use webapp_lib::{
    components::{Button, State},
    define_window_url, handle_ws_connection,
};

use webapp::{
    api, process_ws_message, ApplicationShell, GlobalNavigation, GlobalState,
};

#[component]
fn App() -> impl IntoView {
    let gs = use_context::<GlobalState>().expect("no global state");

    let command_start = move || {
        let msg = Messages::CommandStart(types::Command::default());
        gs.send_msg.set(Some(msg));
    };

    let command_stop = move || {
        let msg = Messages::CommandStop(types::Command::default());
        gs.send_msg.set(Some(msg));
    };

    let motor_state =
        Signal::derive(move || match gs.motor_state.get().value {
            0 => "Стоп".to_string(),
            1 => "Работа".to_string(),
            _ => "???".to_string(),
        });

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
                        <State
                            text=motor_state
                            state_inactive=Signal::derive(move || {
                                gs.motor_state.get().value == 0
                            })
                            state_active=Signal::derive(move || {
                                gs.motor_state.get().value == 1
                            })
                            state_warning=Signal::default()
                            state_error=Signal::default()
                        />
                    </p>
                </div>
            </div>
            <div class="flex flex-row">
                <div class="basis-1/2">
                    <p class="m-4">
                        Температура
                    </p>
                </div>
                <div class="basis-1/2">
                    <p class="m-4">
                        { Signal::derive(move|| gs.temperature.get().value) }
                    </p>
                </div>
            </div>
            <div class="flex flex-row">
                <div class="basis-1/2">
                    <Button
                    label="Start".to_string()
                    on_click=command_start
                    />
                </div>
                <div class="basis-1/2">
                    <Button
                    label="Stop".to_string()
                    on_click=command_stop
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn App2() -> impl IntoView {
    view! {
        <i class="fa-brands fa-github-square"></i>
    }
}

pub fn main() {
    provide_context(GlobalState::new());
    let global_state = use_context::<GlobalState>().expect("no global state");

    let window_url =
        define_window_url().expect("Не удалось определить URL окна");
    global_state.window_url.set(window_url.clone());

    let api_url = format!("http://{}:3001/value/", window_url.host().unwrap());
    global_state.api_url.set(api_url);

    create_resource(
        move || (global_state.send_msg.get(), global_state.api_url.get()),
        |(send_msg, api_url)| async move {
            if let Some(send_msg) = send_msg {
                api::send_message_to_api(&api_url, send_msg).await;
            }
        },
    );

    let ws_url = format!("ws://{}:8081", window_url.host().unwrap());
    spawn_local(async move {
        handle_ws_connection(&ws_url, process_ws_message).await;
    });

    mount_to_body(|| {
        view! {
            <Router>
                <ApplicationShell
                    navigation=GlobalNavigation
                    content=|| view!(
                        <Routes>
                            <Route path="/" view=App/>
                            <Route path="/app2" view=App2/>
                        </Routes>
                    )
                />
            </Router>
        }
    })
}
