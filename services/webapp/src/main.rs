use leptos::*;

use messages::{self};
use webapp_lib::{define_window_url, handle_ws_connection};

use webapp::{api, process_ws_message, GlobalState};

#[component]
fn App() -> impl IntoView {
    let global_state = use_context::<GlobalState>().expect("no global state");

    let command_start = create_action(|_: &()| async {
        let msg = messages::Messages::CommandStart(
            messages::types::Command::new(None),
        );
        api::send_message_to_api(msg).await;
    });

    let command_stop = create_action(|_: &()| async {
        let msg = messages::Messages::CommandStop(
            messages::types::Command::new(None),
        );
        api::send_message_to_api(msg).await;
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
                            text=move || match global_state.motor_state.get().value {
                                0 => "Стоп".to_string(),
                                1 => "Работа".to_string(),
                                _ => "???".to_string(),
                            }
                            inactive=move || {
                                global_state.motor_state.get().value == 0
                            }
                            active=move || {
                                global_state.motor_state.get().value == 1
                            }
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
                        { move|| {global_state.temperature.get().value}}
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
fn State<TText, TInactive, TActive>(
    text: TText,
    inactive: TInactive,
    active: TActive,
) -> impl IntoView
where
    TText: Fn() -> String + 'static + Copy,
    TInactive: Fn() -> bool + 'static + Copy,
    TActive: Fn() -> bool + 'static + Copy,
{
    view! {
        <span class="inline-flex items-center rounded-md px-2 py-1 text-sm font-semibold ring-1 ring-inset ring-gray-500/10"

        class=("bg-grey-50", inactive)
        class=("text-gray-600", inactive)

        class=("bg-green-50", active)
        class=("text-green-700", active)
        >
            { text }
        </span>
    }
}

pub fn main() {
    provide_context(GlobalState::new());

    let window_url =
        define_window_url().expect("Не удалось определить URL окна");

    let ws_url = format!("ws://{}:8081", window_url.host().unwrap());

    spawn_local(async move {
        handle_ws_connection(&ws_url, process_ws_message).await;
    });

    mount_to_body(|| view! { <App/> })
}
