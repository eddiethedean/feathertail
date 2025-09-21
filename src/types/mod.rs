use pyo3::prelude::*;
use time::{OffsetDateTime, PrimitiveDateTime, Date, Time, Month, Weekday};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum DateTimeColumn {
    DateTime(Vec<OffsetDateTime>),
    OptDateTime(Vec<Option<OffsetDateTime>>),
    NaiveDateTime(Vec<PrimitiveDateTime>),
    OptNaiveDateTime(Vec<Option<PrimitiveDateTime>>),
}

impl DateTimeColumn {
    pub fn len(&self) -> usize {
        match self {
            DateTimeColumn::DateTime(v) => v.len(),
            DateTimeColumn::OptDateTime(v) => v.len(),
            DateTimeColumn::NaiveDateTime(v) => v.len(),
            DateTimeColumn::OptNaiveDateTime(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<PyObject> {
        Python::with_gil(|py| {
            match self {
                DateTimeColumn::DateTime(v) => {
                    if let Some(dt) = v.get(index) {
                        let py_dt = py.import("datetime").ok()?.getattr("datetime").ok()?;
                        let py_datetime = py_dt.call_method1("fromtimestamp", (dt.unix_timestamp(),)).ok()?;
                        Some(py_datetime.into())
                    } else {
                        None
                    }
                },
                DateTimeColumn::OptDateTime(v) => {
                    if let Some(Some(dt)) = v.get(index) {
                        let py_dt = py.import("datetime").unwrap().getattr("datetime").unwrap();
                        let py_datetime = py_dt.call_method1("fromtimestamp", (dt.unix_timestamp(),)).unwrap();
                        Some(py_datetime.into())
                    } else {
                        None
                    }
                },
                DateTimeColumn::NaiveDateTime(v) => {
                    if let Some(dt) = v.get(index) {
                        let py_dt = py.import("datetime").ok()?.getattr("datetime").ok()?;
                        let py_datetime = py_dt.call_method1("fromtimestamp", (dt.assume_utc().unix_timestamp(),)).ok()?;
                        Some(py_datetime.into())
                    } else {
                        None
                    }
                },
                DateTimeColumn::OptNaiveDateTime(v) => {
                    if let Some(Some(dt)) = v.get(index) {
                        let py_dt = py.import("datetime").unwrap().getattr("datetime").unwrap();
                        let py_datetime = py_dt.call_method1("fromtimestamp", (dt.assume_utc().unix_timestamp(),)).unwrap();
                        Some(py_datetime.into())
                    } else {
                        None
                    }
                },
            }
        })
    }

    /// Extract year from DateTime values
    pub fn year(&self) -> Vec<Option<i32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.year())).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.year())).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.year())).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.year())).collect(),
        }
    }

    /// Extract month from DateTime values
    pub fn month(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.month() as u8 as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.month() as u8 as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.month() as u8 as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.month() as u8 as u32)).collect(),
        }
    }

    /// Extract day from DateTime values
    pub fn day(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.day() as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.day() as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.day() as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.day() as u32)).collect(),
        }
    }

    /// Extract hour from DateTime values
    pub fn hour(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.hour() as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.hour() as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.hour() as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.hour() as u32)).collect(),
        }
    }

    /// Extract minute from DateTime values
    pub fn minute(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.minute() as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.minute() as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.minute() as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.minute() as u32)).collect(),
        }
    }

    /// Extract second from DateTime values
    pub fn second(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.second() as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.second() as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.second() as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.second() as u32)).collect(),
        }
    }

    /// Extract day of week (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
    pub fn day_of_week(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| {
                let weekday = dt.weekday() as u8;
                Some(if weekday == 6 { 0 } else { (weekday + 1) as u32 })
            }).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| {
                let weekday = dt.weekday() as u8;
                if weekday == 6 { 0 } else { (weekday + 1) as u32 }
            })).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| {
                let weekday = dt.weekday() as u8;
                Some(if weekday == 6 { 0 } else { (weekday + 1) as u32 })
            }).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| {
                let weekday = dt.weekday() as u8;
                if weekday == 6 { 0 } else { (weekday + 1) as u32 }
            })).collect(),
        }
    }

    /// Extract day of year (1-366)
    pub fn day_of_year(&self) -> Vec<Option<u32>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.ordinal() as u32)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.ordinal() as u32)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.ordinal() as u32)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.ordinal() as u32)).collect(),
        }
    }

    /// Convert to timestamp (seconds since Unix epoch)
    pub fn timestamp(&self) -> Vec<Option<i64>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some(dt.unix_timestamp())).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.unix_timestamp())).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some(dt.assume_utc().unix_timestamp())).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| dt.assume_utc().unix_timestamp())).collect(),
        }
    }

    /// Convert to timestamp with microseconds
    pub fn timestamp_micros(&self) -> Vec<Option<i64>> {
        match self {
            DateTimeColumn::DateTime(v) => v.iter().map(|dt| Some((dt.unix_timestamp_nanos() / 1_000_000) as i64)).collect(),
            DateTimeColumn::OptDateTime(v) => v.iter().map(|opt| opt.map(|dt| (dt.unix_timestamp_nanos() / 1_000_000) as i64)).collect(),
            DateTimeColumn::NaiveDateTime(v) => v.iter().map(|dt| Some((dt.assume_utc().unix_timestamp_nanos() / 1_000_000) as i64)).collect(),
            DateTimeColumn::OptNaiveDateTime(v) => v.iter().map(|opt| opt.map(|dt| (dt.assume_utc().unix_timestamp_nanos() / 1_000_000) as i64)).collect(),
        }
    }
}
