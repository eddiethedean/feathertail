use crate::frame::{TinyFrame, TinyColumn};
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyFloat, PyString, PyBool};

pub fn cast_column_impl(frame: &mut TinyFrame, py: Python, column_name: String, new_type: &PyAny) -> PyResult<()> {
    let col = frame.columns.get_mut(&column_name).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column_name))
    })?;

    // Block casting on Mixed and OptMixed columns
    if matches!(col, TinyColumn::Mixed(_) | TinyColumn::OptMixed(_)) {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Cannot cast Mixed or OptMixed columns containing fallback Python objects",
        ));
    }

    let is_str = new_type.is(py.get_type::<PyString>());
    let is_int = new_type.is(py.get_type::<PyInt>());
    let is_float = new_type.is(py.get_type::<PyFloat>());
    let is_bool = new_type.is(py.get_type::<PyBool>());

    *col = match col {
        TinyColumn::Int(vec) => {
            if is_str {
                TinyColumn::Str(vec.iter().map(|v| v.to_string()).collect())
            } else if is_float {
                TinyColumn::Float(vec.iter().map(|v| *v as f64).collect())
            } else if is_bool {
                TinyColumn::Bool(vec.iter().map(|v| *v != 0).collect())
            } else if is_int {
                TinyColumn::Int(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::Float(vec) => {
            if is_str {
                TinyColumn::Str(vec.iter().map(|v| v.to_string()).collect())
            } else if is_int {
                TinyColumn::Int(vec.iter().map(|v| *v as i64).collect())
            } else if is_bool {
                TinyColumn::Bool(vec.iter().map(|v| *v != 0.0).collect())
            } else if is_float {
                TinyColumn::Float(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::Bool(vec) => {
            if is_str {
                TinyColumn::Str(vec.iter().map(|v| v.to_string()).collect())
            } else if is_int {
                TinyColumn::Int(vec.iter().map(|v| if *v { 1 } else { 0 }).collect())
            } else if is_float {
                TinyColumn::Float(vec.iter().map(|v| if *v { 1.0 } else { 0.0 }).collect())
            } else if is_bool {
                TinyColumn::Bool(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::Str(vec) => {
            if is_int {
                TinyColumn::Int(vec.iter().map(|s| s.parse::<i64>().unwrap_or(0)).collect())
            } else if is_float {
                TinyColumn::Float(vec.iter().map(|s| s.parse::<f64>().unwrap_or(0.0)).collect())
            } else if is_bool {
                TinyColumn::Bool(vec.iter().map(|s| !s.is_empty()).collect())
            } else if is_str {
                TinyColumn::Str(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::OptInt(vec) => {
            if is_str {
                TinyColumn::OptStr(vec.iter().map(|o| o.map(|v| v.to_string())).collect())
            } else if is_float {
                TinyColumn::OptFloat(vec.iter().map(|o| o.map(|v| v as f64)).collect())
            } else if is_bool {
                TinyColumn::OptBool(vec.iter().map(|o| o.map(|v| v != 0)).collect())
            } else if is_int {
                TinyColumn::OptInt(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::OptFloat(vec) => {
            if is_str {
                TinyColumn::OptStr(vec.iter().map(|o| o.map(|v| v.to_string())).collect())
            } else if is_int {
                TinyColumn::OptInt(vec.iter().map(|o| o.map(|v| v as i64)).collect())
            } else if is_bool {
                TinyColumn::OptBool(vec.iter().map(|o| o.map(|v| v != 0.0)).collect())
            } else if is_float {
                TinyColumn::OptFloat(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::OptBool(vec) => {
            if is_str {
                TinyColumn::OptStr(vec.iter().map(|o| o.map(|v| v.to_string())).collect())
            } else if is_int {
                TinyColumn::OptInt(vec.iter().map(|o| o.map(|v| if v { 1 } else { 0 })).collect())
            } else if is_float {
                TinyColumn::OptFloat(vec.iter().map(|o| o.map(|v| if v { 1.0 } else { 0.0 })).collect())
            } else if is_bool {
                TinyColumn::OptBool(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        TinyColumn::OptStr(vec) => {
            if is_int {
                TinyColumn::OptInt(vec.iter().map(|o| o.as_ref().and_then(|s| s.parse::<i64>().ok())).collect())
            } else if is_float {
                TinyColumn::OptFloat(vec.iter().map(|o| o.as_ref().and_then(|s| s.parse::<f64>().ok())).collect())
            } else if is_bool {
                TinyColumn::OptBool(vec.iter().map(|o| o.as_ref().map(|s| !s.is_empty())).collect())
            } else if is_str {
                TinyColumn::OptStr(vec.clone())
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Unsupported target type"));
            }
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported column type for casting",
            ));
        }
    };

    Ok(())
}
