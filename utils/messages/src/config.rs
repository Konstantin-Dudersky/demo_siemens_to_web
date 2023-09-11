use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};

use crate::types;
use crate::Errors;

/// Все сообщения в системе
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Messages {
    MotorState(types::SingleValue<i16>),
    CommandStart(types::Command),
    CommandStop(types::Command),
    SetpointRead(types::SingleValue<f64>),
    SetpointWrite(types::SingleValue<f64>),
    Temperature(types::SingleValue<f64>),
}

impl Messages {
    /// Ключ для сохранения в базе данных
    pub fn key(&self) -> String {
        let full_str = self.to_string();
        let parenth_index = full_str.find("(");
        let full_str: String = match parenth_index {
            Some(value) => full_str.chars().take(value).collect(),
            None => full_str,
        };
        full_str
    }

    pub fn deserialize(message: &str) -> Result<Self, Errors> {
        match deserialize::<Self>(message) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Errors::DeserializationError(error))
            }
        }
    }

    pub fn serialize(&self) -> Result<String, Errors> {
        match serialize(&self) {
            Ok(value) => Ok(value),
            Err(error) => {
                let error = error.to_string();
                Err(Errors::SerializationError(error))
            }
        }
    }
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// test ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let msg1 = Messages::MotorState(types::SingleValue::new(10, None));
        assert_eq!("MotorState", msg1.key());
    }

    #[test]
    fn ser_deser() {
        let msg1 = Messages::MotorState(types::SingleValue::new(10, None));

        let ser = msg1.serialize().unwrap();
        let deser = Messages::deserialize(&ser).unwrap();

        assert_eq!(msg1, deser);
    }
}
