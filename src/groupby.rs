use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::collections::HashMap;
use crate::frame::{TinyColumn, TinyFrame};

#[pyclass]
pub struct TinyGroupBy {
    #[pyo3(get)]
    pub keys: Vec<String>,
    groups: HashMap<Vec<Option<String>>, Vec<usize>>,
}

#[pymethods]
impl TinyGroupBy {
    #[new]
    fn new(frame: &TinyFrame, keys: Vec<String>) -> PyResult<Self> {
        let mut groups = HashMap::new();
        let mut key_columns = Vec::new();

        for key in &keys {
            let col = frame.columns.get(key).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Key column '{}' not found", key))
            })?;

            // For now, only string columns can be used as keys
            let col_strings: Vec<Option<String>> = match col {
                TinyColumn::Str(v) => v.iter().map(|s| Some(s.clone())).collect(),
                TinyColumn::OptStr(v) => v.clone(),
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Only string columns can be used as group keys for now",
                    ));
                }
            };

            key_columns.push(col_strings);
        }

        for row_idx in 0..frame.length {
            let mut key = Vec::new();
            for col in &key_columns {
                key.push(col[row_idx].clone());
            }
            groups.entry(key).or_insert_with(Vec::new).push(row_idx);
        }

        Ok(TinyGroupBy { keys, groups })
    }

    fn count(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        self.aggregate(frame, "count")
    }

    fn sum(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "sum")
    }

    fn mean(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "mean")
    }

    fn min(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "min")
    }

    fn max(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "max")
    }

    fn std(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "std")
    }

    fn var(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "var")
    }

    fn median(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "median")
    }

    fn first(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "first")
    }

    fn last(&self, frame: &TinyFrame, column: String) -> PyResult<TinyFrame> {
        self.aggregate_column(frame, column, "last")
    }

    fn size(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        self.aggregate(frame, "size")
    }

    #[getter]
    fn groups(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        for (key_vec, val) in &self.groups {
            let py_key = PyTuple::new(
                py,
                key_vec.iter().map(|v| v.clone().map_or(py.None(), |s| s.into_py(py))),
            );
            dict.set_item(py_key, val)?;
        }

        Ok(dict.into())
    }
}

impl TinyGroupBy {
    // Helper method for basic aggregations (count, size)
    fn aggregate(&self, frame: &TinyFrame, agg_type: &str) -> PyResult<TinyFrame> {
        let mut columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut key_columns: Vec<Vec<Option<String>>> = vec![Vec::new(); self.keys.len()];
        let mut agg_column: Vec<Option<i64>> = Vec::new();

        for (key, row_indices) in &self.groups {
            for (i, val) in key.iter().enumerate() {
                key_columns[i].push(val.clone());
            }
            agg_column.push(Some(row_indices.len() as i64));
        }

        for (i, key_name) in self.keys.iter().enumerate() {
            columns.insert(key_name.clone(), TinyColumn::OptStr(key_columns[i].clone()));
        }
        columns.insert(agg_type.to_string(), TinyColumn::OptInt(agg_column));

        Ok(TinyFrame {
            columns,
            length: self.groups.len(),
            py_objects: frame.py_objects.clone(),
        })
    }

    // Helper method for column-specific aggregations
    fn aggregate_column(&self, frame: &TinyFrame, column_name: String, agg_type: &str) -> PyResult<TinyFrame> {
        let column = frame.columns.get(&column_name).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column_name))
        })?;

        let mut columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut key_columns: Vec<Vec<Option<String>>> = vec![Vec::new(); self.keys.len()];
        let mut agg_values: Vec<Option<f64>> = Vec::new();

        for (key, row_indices) in &self.groups {
            for (i, val) in key.iter().enumerate() {
                key_columns[i].push(val.clone());
            }

            let agg_value = match agg_type {
                "sum" => self.calculate_sum(column, row_indices),
                "mean" => self.calculate_mean(column, row_indices),
                "min" => self.calculate_min(column, row_indices),
                "max" => self.calculate_max(column, row_indices),
                "std" => self.calculate_std(column, row_indices),
                "var" => self.calculate_var(column, row_indices),
                "median" => self.calculate_median(column, row_indices),
                "first" => self.calculate_first(column, row_indices),
                "last" => self.calculate_last(column, row_indices),
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown aggregation type: {}", agg_type)
                )),
            };

            agg_values.push(agg_value);
        }

        for (i, key_name) in self.keys.iter().enumerate() {
            columns.insert(key_name.clone(), TinyColumn::OptStr(key_columns[i].clone()));
        }
        columns.insert(format!("{}_{}", column_name, agg_type), TinyColumn::OptFloat(agg_values));

        Ok(TinyFrame {
            columns,
            length: self.groups.len(),
            py_objects: frame.py_objects.clone(),
        })
    }

}

impl TinyGroupBy {
    // Aggregation calculation methods (private helper methods)
    fn calculate_sum(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => {
                let sum: i64 = row_indices.iter().map(|&i| v[i]).sum();
                Some(sum as f64)
            }
            TinyColumn::Float(v) => {
                let sum: f64 = row_indices.iter().map(|&i| v[i]).sum();
                Some(sum)
            }
            TinyColumn::OptInt(v) => {
                let sum: i64 = row_indices.iter().filter_map(|&i| v[i]).sum();
                Some(sum as f64)
            }
            TinyColumn::OptFloat(v) => {
                let sum: f64 = row_indices.iter().filter_map(|&i| v[i]).sum();
                Some(sum)
            }
            _ => None,
        }
    }

    fn calculate_mean(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => {
                let sum: i64 = row_indices.iter().map(|&i| v[i]).sum();
                Some(sum as f64 / row_indices.len() as f64)
            }
            TinyColumn::Float(v) => {
                let sum: f64 = row_indices.iter().map(|&i| v[i]).sum();
                Some(sum / row_indices.len() as f64)
            }
            TinyColumn::OptInt(v) => {
                let values: Vec<i64> = row_indices.iter().filter_map(|&i| v[i]).collect();
                if values.is_empty() {
                    None
                } else {
                    let sum: i64 = values.iter().sum();
                    Some(sum as f64 / values.len() as f64)
                }
            }
            TinyColumn::OptFloat(v) => {
                let values: Vec<f64> = row_indices.iter().filter_map(|&i| v[i]).collect();
                if values.is_empty() {
                    None
                } else {
                    let sum: f64 = values.iter().sum();
                    Some(sum / values.len() as f64)
                }
            }
            _ => None,
        }
    }

    fn calculate_min(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => {
                let min_val = row_indices.iter().map(|&i| v[i]).min()?;
                Some(min_val as f64)
            }
            TinyColumn::Float(v) => {
                row_indices.iter().map(|&i| v[i]).min_by(|a, b| a.partial_cmp(b).unwrap())
            }
            TinyColumn::OptInt(v) => {
                let min_val = row_indices.iter().filter_map(|&i| v[i]).min()?;
                Some(min_val as f64)
            }
            TinyColumn::OptFloat(v) => {
                row_indices.iter().filter_map(|&i| v[i]).min_by(|a, b| a.partial_cmp(b).unwrap())
            }
            _ => None,
        }
    }

    fn calculate_max(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => {
                let max_val = row_indices.iter().map(|&i| v[i]).max()?;
                Some(max_val as f64)
            }
            TinyColumn::Float(v) => {
                row_indices.iter().map(|&i| v[i]).max_by(|a, b| a.partial_cmp(b).unwrap())
            }
            TinyColumn::OptInt(v) => {
                let max_val = row_indices.iter().filter_map(|&i| v[i]).max()?;
                Some(max_val as f64)
            }
            TinyColumn::OptFloat(v) => {
                row_indices.iter().filter_map(|&i| v[i]).max_by(|a, b| a.partial_cmp(b).unwrap())
            }
            _ => None,
        }
    }

    fn calculate_std(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        let mean = self.calculate_mean(column, row_indices)?;
        let variance = self.calculate_variance_with_mean(column, row_indices, mean)?;
        Some(variance.sqrt())
    }

    fn calculate_var(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        let mean = self.calculate_mean(column, row_indices)?;
        self.calculate_variance_with_mean(column, row_indices, mean)
    }

    fn calculate_variance_with_mean(&self, column: &TinyColumn, row_indices: &[usize], mean: f64) -> Option<f64> {
        let values: Vec<f64> = match column {
            TinyColumn::Int(v) => row_indices.iter().map(|&i| v[i] as f64).collect(),
            TinyColumn::Float(v) => row_indices.iter().map(|&i| v[i]).collect(),
            TinyColumn::OptInt(v) => row_indices.iter().filter_map(|&i| v[i]).map(|x| x as f64).collect(),
            TinyColumn::OptFloat(v) => row_indices.iter().filter_map(|&i| v[i]).collect(),
            _ => return None,
        };

        if values.is_empty() {
            return None;
        }

        let variance = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        Some(variance)
    }

    fn calculate_median(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        let mut values: Vec<f64> = match column {
            TinyColumn::Int(v) => row_indices.iter().map(|&i| v[i] as f64).collect(),
            TinyColumn::Float(v) => row_indices.iter().map(|&i| v[i]).collect(),
            TinyColumn::OptInt(v) => row_indices.iter().filter_map(|&i| v[i]).map(|x| x as f64).collect(),
            TinyColumn::OptFloat(v) => row_indices.iter().filter_map(|&i| v[i]).collect(),
            _ => return None,
        };

        if values.is_empty() {
            return None;
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = values.len() / 2;
        if values.len() % 2 == 0 {
            Some((values[mid - 1] + values[mid]) / 2.0)
        } else {
            Some(values[mid])
        }
    }

    fn calculate_first(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        if row_indices.is_empty() {
            return None;
        }
        let first_idx = row_indices[0];
        match column {
            TinyColumn::Int(v) => Some(v[first_idx] as f64),
            TinyColumn::Float(v) => Some(v[first_idx]),
            TinyColumn::OptInt(v) => v[first_idx].map(|x| x as f64),
            TinyColumn::OptFloat(v) => v[first_idx],
            _ => None,
        }
    }

    fn calculate_last(&self, column: &TinyColumn, row_indices: &[usize]) -> Option<f64> {
        if row_indices.is_empty() {
            return None;
        }
        let last_idx = row_indices[row_indices.len() - 1];
        match column {
            TinyColumn::Int(v) => Some(v[last_idx] as f64),
            TinyColumn::Float(v) => Some(v[last_idx]),
            TinyColumn::OptInt(v) => v[last_idx].map(|x| x as f64),
            TinyColumn::OptFloat(v) => v[last_idx],
            _ => None,
        }
    }
}
