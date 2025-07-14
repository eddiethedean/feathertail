use crate::frame::{TinyFrame, TinyColumn};
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyFloat, PyString, PyBool};

pub fn cast_column_impl(frame: &mut TinyFrame, py: Python, column_name: String, new_type: &PyAny) -> PyResult<()> {
    let col = frame.columns.get_mut(&column_name).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column_name))
    })?;

    // 🔒 Block casting on Mixed and OptMixed columns
    if matches!(col, TinyColumn::Mixed(_) | TinyColumn::OptMixed(_)) {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Cannot cast Mixed or OptMixed columns containing fallback Python objects",
        ));
    }

    if new_type.is(py.get_type::<PyString>()) {
        match col {
            TinyColumn::OptStr(_) | TinyColumn::Str(_) => {}
            TinyColumn::OptInt(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|v| v.to_string())).collect();
                *col = TinyColumn::OptStr(new_vec);
            }
            TinyColumn::OptFloat(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|v| v.to_string())).collect();
                *col = TinyColumn::OptStr(new_vec);
            }
            TinyColumn::OptBool(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|v| v.to_string())).collect();
                *col = TinyColumn::OptStr(new_vec);
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Only optional columns can be cast currently"));
            }
        }
    } else if new_type.is(py.get_type::<PyInt>()) {
        match col {
            TinyColumn::OptInt(_) => {}
            TinyColumn::OptFloat(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|f| f as i64)).collect();
                *col = TinyColumn::OptInt(new_vec);
            }
            TinyColumn::OptBool(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|b| if b { 1 } else { 0 })).collect();
                *col = TinyColumn::OptInt(new_vec);
            }
            TinyColumn::OptStr(vec) => {
                let new_vec = vec.iter().map(|opt| opt.as_ref().and_then(|s| s.parse::<i64>().ok())).collect();
                *col = TinyColumn::OptInt(new_vec);
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Only optional columns can be cast currently"));
            }
        }
    } else if new_type.is(py.get_type::<PyFloat>()) {
        match col {
            TinyColumn::OptFloat(_) => {}
            TinyColumn::OptInt(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|i| i as f64)).collect();
                *col = TinyColumn::OptFloat(new_vec);
            }
            TinyColumn::OptBool(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|b| if b { 1.0 } else { 0.0 })).collect();
                *col = TinyColumn::OptFloat(new_vec);
            }
            TinyColumn::OptStr(vec) => {
                let new_vec = vec.iter().map(|opt| opt.as_ref().and_then(|s| s.parse::<f64>().ok())).collect();
                *col = TinyColumn::OptFloat(new_vec);
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Only optional columns can be cast currently"));
            }
        }
    } else if new_type.is(py.get_type::<PyBool>()) {
        match col {
            TinyColumn::OptBool(_) => {}
            TinyColumn::OptInt(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|i| i != 0)).collect();
                *col = TinyColumn::OptBool(new_vec);
            }
            TinyColumn::OptFloat(vec) => {
                let new_vec = vec.iter().map(|opt| opt.map(|f| f != 0.0)).collect();
                *col = TinyColumn::OptBool(new_vec);
            }
            TinyColumn::OptStr(vec) => {
                let new_vec = vec.iter().map(|opt| opt.as_ref().map(|s| !s.is_empty())).collect();
                *col = TinyColumn::OptBool(new_vec);
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Only optional columns can be cast currently"));
            }
        }
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported type: pass int, float, str, or bool",
        ));
    }

    Ok(())
}
