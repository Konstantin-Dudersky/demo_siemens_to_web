use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleValue<T> {
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Messages {
    IntValueFromOpcUa(SimpleValue<i16>),
    CommandStart(SimpleValue<bool>),
    CommandStop(SimpleValue<bool>),
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
        let msg1 = Messages::IntValueFromOpcUa(SimpleValue { value: 10 });
        assert_eq!("IntValueFromOpcUa", msg1.key());
    }
}
