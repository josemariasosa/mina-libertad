use std::{time::{SystemTime, UNIX_EPOCH}, fmt};
use crate::types::EpochMillis;

use chrono::{Utc, NaiveDate};

pub struct Now {
    nanosecs: u64
}

impl Now {
    pub fn new() -> Self {
        Self {
            nanosecs: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64
        }
    }

    pub fn new_from_epoch_millis(epoch_millis: EpochMillis) -> Self {
        Self { nanosecs: (epoch_millis * 1_000_000) as u64 }
    }

    /// TODO: This converts a date to a timestamp at 13:00 hrs UTC / 07:00 hrs CST.
    pub fn new_from_datetime_str(datetime: &str, format: &str) -> Self {
        let epoch_millis = if format == "%Y-%m-%d" {
            let dt = NaiveDate::parse_from_str(datetime, format)
                .unwrap()
                .and_hms_milli_opt(13, 0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            dt.timestamp_millis() as EpochMillis
        } else {
            unimplemented!();
        };

        Now::new_from_epoch_millis(epoch_millis)
    }

    pub fn get_millis_since(epoch_millis: EpochMillis) -> EpochMillis {
        Now::new().to_epoch_millis() - epoch_millis
    }

    pub fn to_epoch_millis(&self) -> EpochMillis {
        (self.nanosecs / 1_000_000) as EpochMillis
    }

    pub fn to_nanos(&self) -> u64 {
        self.nanosecs
    }

    pub fn increment_min(&self, min: u64) -> Now {
        Now { nanosecs: self.nanosecs + (min * 60 * 1_000_000_000) }
    }

    pub fn increment_days(&self, days: u64) -> Now {
        Now { nanosecs: self.nanosecs + (days * 24 * 60 * 60 * 1_000_000_000) }
    }

    pub fn reduce_min(&self, min: u64) -> Now {
        Now { nanosecs: self.nanosecs - (min * 60 * 1_000_000_000) }
    }
}

impl Copy for Now {}

impl Clone for Now {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Display for Now {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_epoch_millis())
    }
}
