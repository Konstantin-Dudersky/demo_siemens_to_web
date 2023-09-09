use chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Type};

#[derive(Type, Clone)]
#[sqlx(type_name = "agg_type", rename_all = "lowercase")]
pub enum AggType {
    Curr,
    First,
    Inc,
    Sum,
    Mean,
    Min,
    Max,
}

#[derive(FromRow)]
pub struct Row {
    pub ts: DateTime<FixedOffset>,
    pub entity: String,
    pub attr: String,
    pub value: Option<f64>,
    pub agg: AggType,
    pub aggts: Option<DateTime<FixedOffset>>,
    pub aggnext: Option<Vec<AggType>>,
}

impl Row {
    pub fn new(
        ts: DateTime<FixedOffset>,
        entity: &str,
        attr: &str,
        value: f64,
    ) -> Self {
        Self {
            ts: ts,
            entity: entity.to_string(),
            attr: attr.to_string(),
            value: Some(value),
            agg: AggType::Curr,
            aggts: None,
            aggnext: None,
        }
    }
}
