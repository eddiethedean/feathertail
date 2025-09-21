use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};
use std::collections::HashMap;

/// Validation operations for TinyFrame
pub struct ValidationOps;

impl ValidationOps {
    /// Validate that all values in a column are not null
    pub fn validate_not_null_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut validation_results = Vec::new();
        let mut _error_count = 0;

        match col {
            TinyColumn::Int(_) | TinyColumn::Float(_) | TinyColumn::Str(_) | TinyColumn::Bool(_) => {
                // Non-optional columns are always not null
                validation_results = vec![true; frame.length];
            },
            TinyColumn::OptInt(v) => {
                for val in v {
                    let is_valid = val.is_some();
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptFloat(v) => {
                for val in v {
                    let is_valid = val.is_some();
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptStr(v) => {
                for val in v {
                    let is_valid = val.is_some();
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptBool(v) => {
                for val in v {
                    let is_valid = val.is_some();
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Validation not supported for this column type"
                ));
            }
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_not_null", column), TinyColumn::Bool(validation_results));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Validate that all values in a numeric column are within a range
    pub fn validate_range_impl(frame: &TinyFrame, column: &str, min: Option<f64>, max: Option<f64>) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut validation_results = Vec::new();
        let mut _error_count = 0;

        match col {
            TinyColumn::Int(v) => {
                for val in v {
                    let val_f64 = *val as f64;
                    let is_valid = (min.is_none() || val_f64 >= min.unwrap()) && 
                                  (max.is_none() || val_f64 <= max.unwrap());
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::Float(v) => {
                for val in v {
                    let is_valid = (min.is_none() || *val >= min.unwrap()) && 
                                  (max.is_none() || *val <= max.unwrap());
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptInt(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => {
                            let val_f64 = *val as f64;
                            (min.is_none() || val_f64 >= min.unwrap()) && 
                            (max.is_none() || val_f64 <= max.unwrap())
                        },
                        None => true, // null values are considered valid for range validation
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptFloat(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => {
                            (min.is_none() || *val >= min.unwrap()) && 
                            (max.is_none() || *val <= max.unwrap())
                        },
                        None => true, // null values are considered valid for range validation
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Range validation only supported for numeric columns"
                ));
            }
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_in_range", column), TinyColumn::Bool(validation_results));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Validate that all values in a string column match a pattern
    pub fn validate_pattern_impl(frame: &TinyFrame, column: &str, pattern: &str) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut validation_results = Vec::new();
        let mut _error_count = 0;

        match col {
            TinyColumn::Str(v) => {
                for val in v {
                    let is_valid = val.contains(pattern);
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptStr(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => val.contains(pattern),
                        None => true, // null values are considered valid for pattern validation
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Pattern validation only supported for string columns"
                ));
            }
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_matches_pattern", column), TinyColumn::Bool(validation_results));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Validate that all values in a column are unique
    pub fn validate_unique_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut validation_results = Vec::new();
        let mut seen_values = std::collections::HashSet::new();
        let mut _error_count = 0;

        match col {
            TinyColumn::Int(v) => {
                for val in v {
                    let is_valid = seen_values.insert(ValueEnum::Int(*val));
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::Float(v) => {
                for val in v {
                    let is_valid = seen_values.insert(ValueEnum::Float(*val));
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::Str(v) => {
                for val in v {
                    let is_valid = seen_values.insert(ValueEnum::Str(val.clone()));
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::Bool(v) => {
                for val in v {
                    let is_valid = seen_values.insert(ValueEnum::Bool(*val));
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptInt(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => seen_values.insert(ValueEnum::Int(*val)),
                        None => true, // null values are considered valid for uniqueness
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptFloat(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => seen_values.insert(ValueEnum::Float(*val)),
                        None => true, // null values are considered valid for uniqueness
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptStr(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => seen_values.insert(ValueEnum::Str(val.clone())),
                        None => true, // null values are considered valid for uniqueness
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            TinyColumn::OptBool(v) => {
                for val in v {
                    let is_valid = match val {
                        Some(val) => seen_values.insert(ValueEnum::Bool(*val)),
                        None => true, // null values are considered valid for uniqueness
                    };
                    if !is_valid { _error_count += 1; }
                    validation_results.push(is_valid);
                }
            },
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Uniqueness validation not supported for this column type"
                ));
            }
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_unique", column), TinyColumn::Bool(validation_results));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Get validation summary for a column
    pub fn validation_summary_impl(frame: &TinyFrame, column: &str) -> PyResult<HashMap<String, f64>> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut summary = HashMap::new();
        let mut null_count = 0;
        let total_count = frame.length;

        match col {
            TinyColumn::Int(_) | TinyColumn::Float(_) | TinyColumn::Str(_) | TinyColumn::Bool(_) => {
                summary.insert("null_count".to_string(), 0.0);
                summary.insert("null_percentage".to_string(), 0.0);
            },
            TinyColumn::OptInt(v) => {
                null_count = v.iter().filter(|x| x.is_none()).count();
            },
            TinyColumn::OptFloat(v) => {
                null_count = v.iter().filter(|x| x.is_none()).count();
            },
            TinyColumn::OptStr(v) => {
                null_count = v.iter().filter(|x| x.is_none()).count();
            },
            TinyColumn::OptBool(v) => {
                null_count = v.iter().filter(|x| x.is_none()).count();
            },
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Validation summary not supported for this column type"
                ));
            }
        }

        summary.insert("null_count".to_string(), null_count as f64);
        summary.insert("null_percentage".to_string(), (null_count as f64 / total_count as f64) * 100.0);
        summary.insert("total_count".to_string(), total_count as f64);
        summary.insert("non_null_count".to_string(), (total_count - null_count) as f64);

        Ok(summary)
    }
}
