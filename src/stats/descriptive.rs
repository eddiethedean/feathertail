use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn};
use std::collections::{HashMap, HashSet};

/// Calculate skewness for numeric columns
pub fn skew_impl(frame: &TinyFrame, column: &str) -> PyResult<f64> {
    let col = frame.columns.get(column)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column)
        ))?;

    let values = extract_numeric_values_for_stats(col)?;
    if values.len() < 3 {
        return Ok(0.0);
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let std = {
        let variance = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / n;
        variance.sqrt()
    };

    if std == 0.0 {
        return Ok(0.0);
    }

    let skewness = values.iter()
        .map(|&x| ((x - mean) / std).powi(3))
        .sum::<f64>() / n;

    Ok(skewness)
}

/// Calculate kurtosis for numeric columns
pub fn kurtosis_impl(frame: &TinyFrame, column: &str) -> PyResult<f64> {
    let col = frame.columns.get(column)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column)
        ))?;

    let values = extract_numeric_values_for_stats(col)?;
    if values.len() < 4 {
        return Ok(0.0);
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let std = {
        let variance = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / n;
        variance.sqrt()
    };

    if std == 0.0 {
        return Ok(0.0);
    }

    let kurtosis = values.iter()
        .map(|&x| ((x - mean) / std).powi(4))
        .sum::<f64>() / n - 3.0; // Excess kurtosis

    Ok(kurtosis)
}

/// Calculate quantile for numeric columns
pub fn quantile_impl(frame: &TinyFrame, column: &str, q: f64) -> PyResult<f64> {
    if q < 0.0 || q > 1.0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Quantile must be between 0.0 and 1.0"
        ));
    }

    let col = frame.columns.get(column)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column)
        ))?;

    let mut values = extract_numeric_values_for_stats(col)?;
    if values.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Cannot calculate quantile for empty column"
        ));
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = values.len();
    let index = q * (n - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        Ok(values[lower])
    } else {
        let weight = index - lower as f64;
        Ok(values[lower] * (1.0 - weight) + values[upper] * weight)
    }
}

/// Calculate mode for any column type
pub fn mode_impl(frame: &TinyFrame, py: Python, column: &str) -> PyResult<PyObject> {
    let col = frame.columns.get(column)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column)
        ))?;

    let mut counts: HashMap<String, i32> = HashMap::new();
    let mut max_count = 0;
    let mut mode_value = None;

    match col {
        TinyColumn::Int(v) => {
            for &val in v {
                let key = val.to_string();
                let count = counts.entry(key.clone()).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    mode_value = Some(val.into_py(py));
                }
            }
        }
        TinyColumn::Float(v) => {
            for &val in v {
                let key = val.to_string();
                let count = counts.entry(key.clone()).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    mode_value = Some(val.into_py(py));
                }
            }
        }
        TinyColumn::Str(v) => {
            for val in v {
                let count = counts.entry(val.clone()).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    mode_value = Some(val.clone().into_py(py));
                }
            }
        }
        TinyColumn::Bool(v) => {
            for &val in v {
                let key = val.to_string();
                let count = counts.entry(key.clone()).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    mode_value = Some(val.into_py(py));
                }
            }
        }
        TinyColumn::OptInt(v) => {
            for &val in v {
                if let Some(val) = val {
                    let key = val.to_string();
                    let count = counts.entry(key.clone()).or_insert(0);
                    *count += 1;
                    if *count > max_count {
                        max_count = *count;
                        mode_value = Some(val.into_py(py));
                    }
                }
            }
        }
        TinyColumn::OptFloat(v) => {
            for &val in v {
                if let Some(val) = val {
                    let key = val.to_string();
                    let count = counts.entry(key.clone()).or_insert(0);
                    *count += 1;
                    if *count > max_count {
                        max_count = *count;
                        mode_value = Some(val.into_py(py));
                    }
                }
            }
        }
        TinyColumn::OptStr(v) => {
            for val in v {
                if let Some(val) = val {
                    let count = counts.entry(val.clone()).or_insert(0);
                    *count += 1;
                    if *count > max_count {
                        max_count = *count;
                        mode_value = Some(val.clone().into_py(py));
                    }
                }
            }
        }
        TinyColumn::OptBool(v) => {
            for &val in v {
                if let Some(val) = val {
                    let key = val.to_string();
                    let count = counts.entry(key.clone()).or_insert(0);
                    *count += 1;
                    if *count > max_count {
                        max_count = *count;
                        mode_value = Some(val.into_py(py));
                    }
                }
            }
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Mode calculation not supported for this column type"
            ));
        }
    }

    match mode_value {
        Some(val) => Ok(val),
        None => Ok(py.None()),
    }
}

/// Count unique values in a column
pub fn nunique_impl(frame: &TinyFrame, column: &str) -> PyResult<usize> {
    let col = frame.columns.get(column)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column)
        ))?;

    let mut unique_values: HashSet<String> = HashSet::new();

    match col {
        TinyColumn::Int(v) => {
            for &val in v {
                unique_values.insert(val.to_string());
            }
        }
        TinyColumn::Float(v) => {
            for &val in v {
                unique_values.insert(val.to_string());
            }
        }
        TinyColumn::Str(v) => {
            for val in v {
                unique_values.insert(val.clone());
            }
        }
        TinyColumn::Bool(v) => {
            for &val in v {
                unique_values.insert(val.to_string());
            }
        }
        TinyColumn::OptInt(v) => {
            for &val in v {
                if let Some(val) = val {
                    unique_values.insert(val.to_string());
                }
            }
        }
        TinyColumn::OptFloat(v) => {
            for &val in v {
                if let Some(val) = val {
                    unique_values.insert(val.to_string());
                }
            }
        }
        TinyColumn::OptStr(v) => {
            for val in v {
                if let Some(val) = val {
                    unique_values.insert(val.clone());
                }
            }
        }
        TinyColumn::OptBool(v) => {
            for &val in v {
                if let Some(val) = val {
                    unique_values.insert(val.to_string());
                }
            }
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unique count not supported for this column type"
            ));
        }
    }

    Ok(unique_values.len())
}

/// Helper function to extract numeric values for statistical calculations
fn extract_numeric_values_for_stats(col: &TinyColumn) -> PyResult<Vec<f64>> {
    match col {
        TinyColumn::Int(v) => Ok(v.iter().map(|&x| x as f64).collect()),
        TinyColumn::Float(v) => Ok(v.clone()),
        TinyColumn::OptInt(v) => Ok(v.iter().filter_map(|&x| x.map(|v| v as f64)).collect()),
        TinyColumn::OptFloat(v) => Ok(v.iter().filter_map(|&x| x).collect()),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Statistical operations only supported on numeric columns"
        )),
    }
}