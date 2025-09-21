use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn};
use std::collections::HashMap;

/// Rolling window configuration
#[derive(Clone, Debug)]
pub struct RollingWindow {
    window_size: usize,
    min_periods: usize,
}

impl RollingWindow {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            min_periods: window_size,
        }
    }

    pub fn min_periods(mut self, min_periods: usize) -> Self {
        self.min_periods = min_periods;
        self
    }
}

/// Expanding window configuration
#[derive(Clone, Debug)]
pub struct ExpandingWindow {
    min_periods: usize,
}

impl ExpandingWindow {
    pub fn new() -> Self {
        Self { min_periods: 1 }
    }

    pub fn min_periods(mut self, min_periods: usize) -> Self {
        self.min_periods = min_periods;
        self
    }
}

/// Window functions for TinyFrame
pub struct WindowOps;

impl WindowOps {
    /// Calculate rolling mean
    pub fn rolling_mean_impl(frame: &TinyFrame, column: &str, window: RollingWindow) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut result_values = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values = &v[start..=i];
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter().map(|&x| x as f64).collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_slice = &v[start..=i];
                    let non_null_count = window_slice.iter().filter(|&&x| x.is_some()).count();
                    
                    if non_null_count >= window.min_periods {
                        let window_values: Vec<f64> = window_slice.iter()
                            .filter_map(|&x| x)
                            .collect();
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_slice = &v[start..=i];
                    let non_null_count = window_slice.iter().filter(|&&x| x.is_some()).count();
                    
                    if non_null_count >= window.min_periods {
                        let window_values: Vec<f64> = window_slice.iter()
                            .filter_map(|&x| x.map(|v| v as f64))
                            .collect();
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Rolling operations only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_rolling_mean", column), TinyColumn::OptFloat(result_values));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate rolling sum
    pub fn rolling_sum_impl(frame: &TinyFrame, column: &str, window: RollingWindow) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut result_values = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values = &v[start..=i];
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter().map(|&x| x as f64).collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_slice = &v[start..=i];
                    let non_null_count = window_slice.iter().filter(|&&x| x.is_some()).count();
                    
                    if non_null_count >= window.min_periods {
                        let window_values: Vec<f64> = window_slice.iter()
                            .filter_map(|&x| x)
                            .collect();
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter()
                        .filter_map(|&x| x.map(|v| v as f64))
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Rolling operations only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_rolling_sum", column), TinyColumn::OptFloat(result_values));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate rolling standard deviation
    pub fn rolling_std_impl(frame: &TinyFrame, column: &str, window: RollingWindow) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut result_values = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values = &v[start..=i];
                    if window_values.len() >= window.min_periods {
                        let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
                        let variance = window_values.iter()
                            .map(|&x| (x - mean).powi(2))
                            .sum::<f64>() / window_values.len() as f64;
                        result_values.push(Some(variance.sqrt()));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter().map(|&x| x as f64).collect();
                    if window_values.len() >= window.min_periods {
                        let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
                        let variance = window_values.iter()
                            .map(|&x| (x - mean).powi(2))
                            .sum::<f64>() / window_values.len() as f64;
                        result_values.push(Some(variance.sqrt()));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter()
                        .filter_map(|&x| x)
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
                        let variance = window_values.iter()
                            .map(|&x| (x - mean).powi(2))
                            .sum::<f64>() / window_values.len() as f64;
                        result_values.push(Some(variance.sqrt()));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                for i in 0..v.len() {
                    let start = if i + 1 >= window.window_size {
                        i + 1 - window.window_size
                    } else {
                        0
                    };
                    
                    let window_values: Vec<f64> = v[start..=i].iter()
                        .filter_map(|&x| x.map(|v| v as f64))
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
                        let variance = window_values.iter()
                            .map(|&x| (x - mean).powi(2))
                            .sum::<f64>() / window_values.len() as f64;
                        result_values.push(Some(variance.sqrt()));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Rolling operations only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_rolling_std", column), TinyColumn::OptFloat(result_values));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate expanding mean
    pub fn expanding_mean_impl(frame: &TinyFrame, column: &str, window: ExpandingWindow) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut result_values = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                for i in 0..v.len() {
                    let window_values = &v[0..=i];
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter().map(|&x| x as f64).collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter()
                        .filter_map(|&x| x)
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter()
                        .filter_map(|&x| x.map(|v| v as f64))
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum / window_values.len() as f64));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expanding operations only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_expanding_mean", column), TinyColumn::OptFloat(result_values));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate expanding sum
    pub fn expanding_sum_impl(frame: &TinyFrame, column: &str, window: ExpandingWindow) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut result_values = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                for i in 0..v.len() {
                    let window_values = &v[0..=i];
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter().map(|&x| x as f64).collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter()
                        .filter_map(|&x| x)
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                for i in 0..v.len() {
                    let window_values: Vec<f64> = v[0..=i].iter()
                        .filter_map(|&x| x.map(|v| v as f64))
                        .collect();
                    if window_values.len() >= window.min_periods {
                        let sum: f64 = window_values.iter().sum();
                        result_values.push(Some(sum));
                    } else {
                        result_values.push(None);
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expanding operations only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_expanding_sum", column), TinyColumn::OptFloat(result_values));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }
}
