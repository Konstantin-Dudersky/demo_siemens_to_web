use chrono::{DateTime as ChronoDateTime, FixedOffset};
use opcua::types::DateTime;

use crate::errors::Errors;

pub fn datetime_to_chrono(
    opc_dt: DateTime,
) -> Result<ChronoDateTime<FixedOffset>, Errors> {
    let dt_str = opc_dt.to_string();
    let dt_chrono = ChronoDateTime::parse_from_rfc3339(&dt_str)?;
    Ok(dt_chrono)
}
