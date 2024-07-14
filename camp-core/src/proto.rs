use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn utc_to_ts(utc: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: utc.timestamp(),
        nanos: utc.timestamp_subsec_nanos() as i32,
    }
}
