use super::DtosConversionError;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub trait ProtoTimestampExt {
    fn to_chrono(&self) -> Result<chrono::DateTime<Utc>, DtosConversionError>;
    fn from_chrono(datetime: DateTime<Utc>) -> Self;
}

impl ProtoTimestampExt for Option<Timestamp> {
    fn to_chrono(&self) -> Result<chrono::DateTime<Utc>, DtosConversionError> {
        let timestamp = self.as_ref().ok_or(DtosConversionError::Timestamp {
            value: "None".to_string(),
        })?;
        Ok(
            DateTime::from_timestamp(timestamp.seconds, timestamp.nanos as u32).ok_or(
                DtosConversionError::Timestamp {
                    value: timestamp.to_string(),
                },
            )?,
        )
    }

    fn from_chrono(datetime: DateTime<Utc>) -> Self {
        Some(Timestamp {
            seconds: datetime.timestamp(),
            nanos: datetime.timestamp_subsec_nanos() as i32,
        })
    }
}
