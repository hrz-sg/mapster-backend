use chrono::{DateTime, Utc};
use sea_query::Value;
use modql::filter::{IntoSeaError, SeaResult};
use serde_json::Value as JsonValue;

pub fn time_to_sea_value(json_value: JsonValue) -> SeaResult<Value> {
    // Try to get a string
    let s = json_value
        .as_str()
        .ok_or_else(|| IntoSeaError::custom("Invalid JSON datetime"))?;

    // Parse as chrono::DateTime<Utc>
    let datetime = DateTime::parse_from_rfc3339(s)
        .map_err(|e| IntoSeaError::custom(e.to_string()))?
        .with_timezone(&Utc);

    // Return Value
    Ok(Value::ChronoDateTimeUtc(Some(Box::new(datetime))))
}

