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
        let mut columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut key_columns: Vec<Vec<Option<String>>> = vec![Vec::new(); self.keys.len()];
        let mut count_column: Vec<Option<i64>> = Vec::new();

        for (key, row_indices) in &self.groups {
            for (i, val) in key.iter().enumerate() {
                key_columns[i].push(val.clone());
            }
            count_column.push(Some(row_indices.len() as i64));
        }

        for (i, key_name) in self.keys.iter().enumerate() {
            columns.insert(key_name.clone(), TinyColumn::OptStr(key_columns[i].clone()));
        }
        columns.insert("count".to_string(), TinyColumn::OptInt(count_column));

        Ok(TinyFrame {
            columns,
            length: self.groups.len(),
            py_objects: frame.py_objects.clone(),
        })
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
