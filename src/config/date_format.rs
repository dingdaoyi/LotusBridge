use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = date {
        let datetime: DateTime<Utc> = Utc.from_utc_datetime(date);
        let formatted = datetime.format(DATE_FORMAT).to_string();
        serializer.serialize_str(&formatted)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    if let Some(value) = value {
        let datetime = DateTime::parse_from_str(&value, DATE_FORMAT)
            .map_err(serde::de::Error::custom)?;
        Ok(Some(datetime.naive_utc()))
    } else {
        Ok(None)
    }
}
