use sqlx::{query, Pool, Postgres};

use crate::{
    models::{AggType, Row},
    Errors,
};

pub async fn save_row_in_db(
    row: &Row,
    pool: &Pool<Postgres>,
) -> Result<(), Errors> {
    let _ = query!(
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
        row.agg.clone() as AggType,
        row.aggts,
    )
    .execute(pool)
    .await?;
    Ok(())
}
