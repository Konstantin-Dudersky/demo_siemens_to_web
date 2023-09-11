use leptos::*;
use serde_json::from_str as deserialize;

use messages::Messages;

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub temperature: RwSignal<f64>,
    pub motor_state: RwSignal<i16>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            temperature: create_rw_signal(0.0),
            motor_state: create_rw_signal(0),
        }
    }
}

pub fn process_ws_message(msg: &str) {
    let global_state = use_context::<GlobalState>().expect("no global state");
    let msg = deserialize::<Messages>(&msg).unwrap();
    // console::log!(format!("1. {:?}", msg));
    match msg {
        Messages::MotorState(value) => {
            global_state.motor_state.set(value.value)
        }
        Messages::CommandStart(_) => (),
        Messages::CommandStop(_) => (),
        Messages::SetpointRead(_) => todo!(),
        Messages::SetpointWrite(_) => todo!(),
        Messages::Temperature(value) => {
            global_state.temperature.set(value.value)
        }
    };
}
