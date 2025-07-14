use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyLong, PyString};
use pyo3::PyTypeInfo; // <---- this is the key fix!
use crate::frame::ValueEnum;
use std::collections::HashMap;

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
    match col {
        crate::frame::TinyColumn::Int(_) => crate::frame::TinyColumn::Int(Vec::new()),
        crate::frame::TinyColumn::Float(_) => crate::frame::TinyColumn::Float(Vec::new()),
        crate::frame::TinyColumn::Bool(_) => crate::frame::TinyColumn::Bool(Vec::new()),
        crate::frame::TinyColumn::Str(_) => crate::frame::TinyColumn::Str(Vec::new()),
        crate::frame::TinyColumn::OptInt(_) => crate::frame::TinyColumn::OptInt(Vec::new()),
        crate::frame::TinyColumn::OptFloat(_) => crate::frame::TinyColumn::OptFloat(Vec::new()),
        crate::frame::TinyColumn::OptBool(_) => crate::frame::TinyColumn::OptBool(Vec::new()),
        crate::frame::TinyColumn::OptStr(_) => crate::frame::TinyColumn::OptStr(Vec::new()),
        crate::frame::TinyColumn::Mixed(_) => crate::frame::TinyColumn::Mixed(Vec::new()),
        crate::frame::TinyColumn::OptMixed(_) => crate::frame::TinyColumn::OptMixed(Vec::new()),
        crate::frame::TinyColumn::PyObject(_) => crate::frame::TinyColumn::PyObject(Vec::new()),
        crate::frame::TinyColumn::OptPyObject(_) => crate::frame::TinyColumn::OptPyObject(Vec::new()),
    }
}

pub fn append_value(col: &mut crate::frame::TinyColumn, idx: usize, src: &crate::frame::TinyColumn) {
    match (col, src) {
        (crate::frame::TinyColumn::Int(dst), crate::frame::TinyColumn::Int(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::Float(dst), crate::frame::TinyColumn::Float(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::Bool(dst), crate::frame::TinyColumn::Bool(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::Str(dst), crate::frame::TinyColumn::Str(src)) => dst.push(src[idx].clone()),
        (crate::frame::TinyColumn::OptInt(dst), crate::frame::TinyColumn::OptInt(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::OptFloat(dst), crate::frame::TinyColumn::OptFloat(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::OptBool(dst), crate::frame::TinyColumn::OptBool(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::OptStr(dst), crate::frame::TinyColumn::OptStr(src)) => dst.push(src[idx].clone()),
        (crate::frame::TinyColumn::Mixed(dst), crate::frame::TinyColumn::Mixed(src)) => dst.push(src[idx].clone()),
        (crate::frame::TinyColumn::OptMixed(dst), crate::frame::TinyColumn::OptMixed(src)) => dst.push(src[idx].clone()),
        (crate::frame::TinyColumn::PyObject(dst), crate::frame::TinyColumn::PyObject(src)) => dst.push(src[idx]),
        (crate::frame::TinyColumn::OptPyObject(dst), crate::frame::TinyColumn::OptPyObject(src)) => dst.push(src[idx]),
        _ => panic!("Column type mismatch in append_value"),
    }
}
