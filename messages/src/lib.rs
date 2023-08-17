use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleValue<T> {
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Messages {
    IntValueFromOpcUa(SimpleValue<u16>),
}
