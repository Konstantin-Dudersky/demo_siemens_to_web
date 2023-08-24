use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleValue<T> {
    pub value: T,
    pub ts: DateTime<Utc>,
}

impl<T> SimpleValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: value,
            ts: Utc::now(),
        }
    }
}
