use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyLong, PyString};
use pyo3::PyTypeInfo; // <---- this is the key fix!
use crate::frame::ValueEnum;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Total ordering for `f64` via [`f64::total_cmp`], including NaN. Use instead of
/// `partial_cmp(..).unwrap()` for sort/min/max to avoid panics.
#[inline]
pub fn total_cmp_f64(a: &f64, b: &f64) -> Ordering {
    a.total_cmp(b)
}

pub fn convert_pyobject_to_valueenum(py_value: &PyAny, py_objects: &mut HashMap<u64, PyObject>) -> PyResult<ValueEnum> {
    if py_value.is_instance(PyBool::type_object(py_value.py()))? {
        Ok(ValueEnum::Bool(py_value.extract()?))
    } else if py_value.is_instance(PyLong::type_object(py_value.py()))? {
        Ok(ValueEnum::Int(py_value.extract()?))
    } else if py_value.is_instance(PyFloat::type_object(py_value.py()))? {
        Ok(ValueEnum::Float(py_value.extract()?))
    } else if py_value.is_instance(PyString::type_object(py_value.py()))? {
        Ok(ValueEnum::Str(py_value.extract()?))
    } else {
        let id = py_value.as_ptr() as u64;
        py_objects.insert(id, py_value.to_object(py_value.py()));
        Ok(ValueEnum::PyObjectId(id))
    }
}

pub fn pyobject_to_option_valueenum(py_value: &PyAny, py_objects: &mut HashMap<u64, PyObject>) -> PyResult<Option<ValueEnum>> {
    if py_value.is_none() {
        Ok(None)
    } else {
        Ok(Some(convert_pyobject_to_valueenum(py_value, py_objects)?))
    }
}

pub fn empty_like_column(col: &crate::frame::TinyColumn) -> crate::frame::TinyColumn {
    col.empty_same_layout()
}

pub fn append_value(col: &mut crate::frame::TinyColumn, idx: usize, src: &crate::frame::TinyColumn) {
    if col.append_row_strict(src, idx).is_err() {
        panic!("Column type mismatch in append_value");
    }
}
