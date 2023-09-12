use std::str::FromStr;

use leptos::*;
use url::Url;

use messages::{types, Messages};

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub send_msg: RwSignal<Option<Messages>>,
    pub window_url: RwSignal<Url>,
    pub api_url: RwSignal<String>,
    pub temperature: RwSignal<types::SingleValue<f64>>,
    pub motor_state: RwSignal<types::SingleValue<i16>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            send_msg: create_rw_signal(None),
            window_url: create_rw_signal(
                Url::from_str("http://localhost").unwrap(),
            ),
            api_url: create_rw_signal("".to_string()),

            temperature: create_rw_signal(types::SingleValue::default()),
            motor_state: create_rw_signal(types::SingleValue::default()),
        }
    }
}

pub fn process_ws_message(msg: &str) {
    let global_state = use_context::<GlobalState>().expect("no global state");
    let msg = Messages::deserialize(&msg).unwrap();
    match msg {
        Messages::MotorState(value) => global_state.motor_state.set(value),
        Messages::CommandStart(_) => (),
        Messages::CommandStop(_) => (),
        Messages::SetpointRead(_) => (),
        Messages::SetpointWrite(_) => (),
        Messages::Temperature(value) => global_state.temperature.set(value),
    };
}
