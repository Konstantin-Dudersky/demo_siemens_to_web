use serde::{Deserialize, Serialize};

use chrono::{DateTime, FixedOffset, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleValue<T> {
    pub value: T,
    pub ts: DateTime<FixedOffset>,
}

impl<T> SimpleValue<T> {
    pub fn new(value: T, ts: Option<DateTime<FixedOffset>>) -> Self {
        let ts = match ts {
            Some(value) => value,
            None => Utc::now().into(),
        };
        Self {
            value: value,
            ts: ts,
        }
    }
}
