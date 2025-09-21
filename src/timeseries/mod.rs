use pyo3::prelude::*;
use time::{OffsetDateTime, PrimitiveDateTime, Date, Time, Month, Weekday};
use std::collections::HashMap;
use crate::frame::{TinyFrame, TinyColumn};

/// Time series operations for TinyFrame
pub struct TimeSeriesOps;

impl TimeSeriesOps {
    /// Parse datetime strings and convert to timestamps
    pub fn parse_datetime_strings(strings: Vec<String>) -> PyResult<Vec<i64>> {
        let mut timestamps = Vec::new();
        
        for s in strings {
            let timestamp = Self::parse_datetime_string(&s)?;
            timestamps.push(timestamp);
        }
        
        Ok(timestamps)
    }

    /// Parse a single datetime string to timestamp
    fn parse_datetime_string(input: &str) -> PyResult<i64> {
        // Handle empty strings
        if input.is_empty() {
            return Ok(0);
        }

        // Try common datetime formats using time crate
        let formats = [
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap(),
            time::format_description::parse("[year]-[month]-[day]").unwrap(),
            time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap(),
            time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]").unwrap(),
            time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z").unwrap(),
            time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap(),
        ];

        for format in &formats {
            if let Ok(dt) = PrimitiveDateTime::parse(input, format) {
                return Ok(dt.assume_utc().unix_timestamp());
            }
        }

        // If all parsing fails, return 0 instead of error
        Ok(0)
    }

    /// Extract year from datetime timestamps
    pub fn extract_year(timestamps: &[i64]) -> Vec<i32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.year()).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract month from datetime timestamps
    pub fn extract_month(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.month() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract day from datetime timestamps
    pub fn extract_day(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.day() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract hour from datetime timestamps
    pub fn extract_hour(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.hour() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract minute from datetime timestamps
    pub fn extract_minute(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.minute() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract second from datetime timestamps
    pub fn extract_second(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.second() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract day of week from datetime timestamps (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
    pub fn extract_day_of_week(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| {
                        // Convert time crate weekday (Sunday=6) to standard (Sunday=0)
                        let weekday = dt.weekday() as u8;
                        if weekday == 6 { 0 } else { (weekday + 1) as u32 }
                    }).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Extract day of year from datetime timestamps (1-366)
    pub fn extract_day_of_year(timestamps: &[i64]) -> Vec<u32> {
        timestamps.iter()
            .map(|&ts| {
                if ts == 0 {
                    0  // Invalid date should return 0
                } else {
                    OffsetDateTime::from_unix_timestamp(ts).map(|dt| dt.ordinal() as u32).unwrap_or(0)
                }
            })
            .collect()
    }

    /// Calculate time differences between consecutive timestamps
    pub fn calculate_time_diffs(timestamps: &[i64]) -> Vec<Option<i64>> {
        if timestamps.len() < 2 {
            return vec![None; timestamps.len()];
        }

        let mut diffs = vec![None];
        for i in 1..timestamps.len() {
            diffs.push(Some(timestamps[i] - timestamps[i - 1]));
        }
        diffs
    }

    /// Shift timestamps by a specified number of seconds
    pub fn shift_timestamps(timestamps: &[i64], seconds: i64) -> Vec<i64> {
        timestamps.iter().map(|&ts| ts + seconds).collect()
    }
}

// Time series operations for TinyFrame - methods will be added to main TinyFrame impl

impl TimeSeriesOps {
    /// Convert datetime string column to timestamps
    pub fn to_timestamps_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let strings = match col {
            TinyColumn::Str(v) => v.clone(),
            TinyColumn::OptStr(v) => {
                v.iter()
                    .map(|opt| opt.as_ref().map(|s| s.clone()).unwrap_or_else(|| "".to_string()))
                    .collect()
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "to_timestamps only supported on string columns"
            )),
        };

        let timestamps = Self::parse_datetime_strings(strings)?;
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_timestamp", column), TinyColumn::Int(timestamps));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract year from datetime string column
    pub fn dt_year_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let years = Self::extract_year(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_year", column), TinyColumn::Int(years.into_iter().map(|y| y as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract month from datetime string column
    pub fn dt_month_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let months = Self::extract_month(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_month", column), TinyColumn::Int(months.into_iter().map(|m| m as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract day from datetime string column
    pub fn dt_day_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let days = Self::extract_day(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_day", column), TinyColumn::Int(days.into_iter().map(|d| d as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract hour from datetime string column
    pub fn dt_hour_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let hours = Self::extract_hour(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_hour", column), TinyColumn::Int(hours.into_iter().map(|h| h as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract minute from datetime string column
    pub fn dt_minute_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let minutes = Self::extract_minute(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_minute", column), TinyColumn::Int(minutes.into_iter().map(|m| m as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract second from datetime string column
    pub fn dt_second_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let seconds = Self::extract_second(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_second", column), TinyColumn::Int(seconds.into_iter().map(|s| s as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract day of week from datetime string column
    pub fn dt_day_of_week_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let days_of_week = Self::extract_day_of_week(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_day_of_week", column), TinyColumn::Int(days_of_week.into_iter().map(|d| d as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Extract day of year from datetime string column
    pub fn dt_day_of_year_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let days_of_year = Self::extract_day_of_year(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_day_of_year", column), TinyColumn::Int(days_of_year.into_iter().map(|d| d as i64).collect()));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate time differences between consecutive datetime values
    pub fn dt_diff_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let diffs = Self::calculate_time_diffs(&timestamps);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_diff", column), TinyColumn::OptInt(diffs));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Shift datetime values by specified seconds
    pub fn dt_shift_impl(frame: &TinyFrame, column: &str, seconds: i64) -> PyResult<TinyFrame> {
        let timestamps = Self::get_timestamps_from_column(frame, column)?;
        let shifted = Self::shift_timestamps(&timestamps, seconds);

        // Convert back to datetime strings
        let shifted_strings: Vec<String> = shifted.iter()
            .map(|&ts| {
                OffsetDateTime::from_unix_timestamp(ts)
                    .map(|dt| {
                        // Use a simple format string for now
                        format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", 
                            dt.year(), 
                            dt.month() as u8, 
                            dt.day() as u8,
                            dt.hour() as u8,
                            dt.minute() as u8,
                            dt.second() as u8
                        )
                    })
                    .unwrap_or_else(|_| "1970-01-01 00:00:00".to_string())
            })
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_shifted", column), TinyColumn::Str(shifted_strings));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Helper method to get timestamps from a column
    fn get_timestamps_from_column(frame: &TinyFrame, column: &str) -> PyResult<Vec<i64>> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        match col {
            TinyColumn::Str(v) => Self::parse_datetime_strings(v.clone()),
            TinyColumn::OptStr(v) => {
                let strings: Vec<String> = v.iter()
                    .map(|opt| opt.as_ref().map(|s| s.clone()).unwrap_or_else(|| "".to_string()))
                    .collect();
                Self::parse_datetime_strings(strings)
            },
            TinyColumn::Int(v) => Ok(v.clone()), // Already timestamps
            TinyColumn::OptInt(v) => {
                let timestamps: Vec<i64> = v.iter()
                    .map(|opt| opt.unwrap_or(0))
                    .collect();
                Ok(timestamps)
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Time series operations only supported on string or integer columns"
            )),
        }
    }
}
