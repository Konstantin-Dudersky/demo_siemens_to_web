use leptos::*;

use messages::{types, Messages};

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub temperature: RwSignal<types::SingleValue<f64>>,
    pub motor_state: RwSignal<types::SingleValue<i16>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            temperature: create_rw_signal(types::SingleValue::new(0.0, None)),
            motor_state: create_rw_signal(types::SingleValue::new(0, None)),
        }
    }
}

pub fn process_ws_message(msg: &str) {
    let global_state = use_context::<GlobalState>().expect("no global state");
    let msg = Messages::deserialize(&msg).unwrap();
    // console::log!(format!("1. {:?}", msg));
    match msg {
        Messages::MotorState(value) => global_state.motor_state.set(value),
        Messages::CommandStart(_) => (),
        Messages::CommandStop(_) => (),
        Messages::SetpointRead(_) => (),
        Messages::SetpointWrite(_) => (),
        Messages::Temperature(value) => global_state.temperature.set(value),
    };
}
