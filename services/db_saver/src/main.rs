use chrono::{DateTime, FixedOffset, Utc};
use sqlx::postgres::{PgPoolOptions, PgTypeInfo};
use tokio::main;

#[derive(sqlx::Type)]
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

#[derive(sqlx::FromRow)]
struct Row {
    pub ts: DateTime<FixedOffset>,
    pub entity: String,
    pub attr: String,
    value: Option<f64>,
    agg: AggType,
    aggts: Option<DateTime<FixedOffset>>,
    aggnext: Option<Vec<AggType>>,
}

#[main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/db_data_test")
        .await?;

    let row = Row {
        ts: Utc::now().into(),
        entity: "14".to_string(),
        attr: "attr".to_string(),
        value: Some(123f64),
        agg: AggType::Curr,
        aggts: None,
        aggnext: None,
    };

    let _ = sqlx::query!(
        r#"
INSERT INTO raw
VALUES ($1, $2, $3, $4, $5::agg_type, $6)
ON CONFLICT (ts, entity, attr, agg) DO UPDATE
    SET value = excluded.value,
        aggts = excluded.aggts,
        aggnext = excluded.aggnext;"#,
        row.ts,
        row.entity,
        row.attr,
        row.value,
        row.agg as AggType,
        row.aggts,
    )
    .execute(&pool)
    .await?;

    Ok(())
}