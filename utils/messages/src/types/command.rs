use serde::{Deserialize, Serialize};

use chrono::{DateTime, FixedOffset, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub ts: DateTime<FixedOffset>,
}

impl Command {
    pub fn new(ts: Option<DateTime<FixedOffset>>) -> Self {
        let ts = match ts {
            Some(value) => value,
            None => Utc::now().into(),
        };
        Self { ts: ts }
    }
}