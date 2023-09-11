use leptos::*;

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
