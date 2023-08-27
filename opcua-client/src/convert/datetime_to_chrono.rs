use chrono::{DateTime as ChronoDateTime, FixedOffset};
use opcua::types::DateTime;

pub fn datetime_to_chrono(
    opc_dt: Option<DateTime>,
) -> Option<ChronoDateTime<FixedOffset>> {
    match opc_dt {
        Some(value) => {
            let dt_str = value.to_string();
            let dt_chrono =
                ChronoDateTime::parse_from_rfc3339(&dt_str).unwrap();
            Some(dt_chrono)
        }
        None => None,
    }
}
