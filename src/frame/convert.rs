use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use std::collections::{HashMap, HashSet};
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};

pub fn from_dicts_impl(py: Python, records: &PyAny) -> PyResult<TinyFrame> {
    let records_list: Vec<&PyDict> = records.extract()?;
    if records_list.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Input list is empty"));
    }

    let mut col_names: Vec<String> = Vec::new();
    for key in records_list[0].keys() {
        col_names.push(key.extract::<String>()?);
    }

    let num_rows = records_list.len();
    let mut columns: HashMap<String, TinyColumn> = HashMap::new();
    let mut py_objects: HashMap<u64, PyObject> = HashMap::new();

    for col in &col_names {
        let mut types_present = HashSet::new();
        let mut has_none = false;
        let mut value_enum_vals: Vec<Option<ValueEnum>> = Vec::new();

        for row in &records_list {
            let val = match row.get_item(col) {
                Ok(Some(v)) => v,
                Ok(None) => {
                    has_none = true;
                    value_enum_vals.push(None);
                    continue;
                }
                Err(e) => return Err(e),
            };

            if val.is_none() {
                has_none = true;
                value_enum_vals.push(None);
                continue;
            }

            if val.is_instance(py.get_type::<pyo3::types::PyBool>())? {
                types_present.insert("Bool");
                value_enum_vals.push(Some(ValueEnum::Bool(val.extract()?)));
            } else if val.is_instance(py.get_type::<pyo3::types::PyLong>())? {
                types_present.insert("Int");
                value_enum_vals.push(Some(ValueEnum::Int(val.extract()?)));
            } else if val.is_instance(py.get_type::<pyo3::types::PyFloat>())? {
                types_present.insert("Float");
                value_enum_vals.push(Some(ValueEnum::Float(val.extract()?)));
            } else if val.is_instance(py.get_type::<pyo3::types::PyString>())? {
                types_present.insert("Str");
                value_enum_vals.push(Some(ValueEnum::Str(val.extract()?)));
            } else {
                // Fallback to PyObject id
                let obj_id = val.getattr("__hash__").ok().and_then(|h| h.extract::<u64>().ok()).unwrap_or_else(|| id_fallback(&val));
                py_objects.insert(obj_id, val.into());
                types_present.insert("PyObject");
                value_enum_vals.push(Some(ValueEnum::PyObjectId(obj_id)));
            }
        }

        let final_col = if types_present.len() == 1 && !has_none {
            match types_present.iter().next().unwrap().as_ref() {
                "Int" => TinyColumn::Int(value_enum_vals.iter().map(|x| match x { Some(ValueEnum::Int(i)) => *i, _ => unreachable!() }).collect()),
                "Float" => TinyColumn::Float(value_enum_vals.iter().map(|x| match x { Some(ValueEnum::Float(f)) => *f, _ => unreachable!() }).collect()),
                "Bool" => TinyColumn::Bool(value_enum_vals.iter().map(|x| match x { Some(ValueEnum::Bool(b)) => *b, _ => unreachable!() }).collect()),
                "Str" => TinyColumn::Str(value_enum_vals.iter().map(|x| match x { Some(ValueEnum::Str(s)) => s.clone(), _ => unreachable!() }).collect()),
                "PyObject" => TinyColumn::PyObject(value_enum_vals.iter().map(|x| match x { Some(ValueEnum::PyObjectId(id)) => *id, _ => unreachable!() }).collect()),
                _ => unreachable!(),
            }
        } else if types_present.len() == 1 && has_none {
            match types_present.iter().next().unwrap().as_ref() {
                "Int" => TinyColumn::OptInt(value_enum_vals.iter().map(|x| x.as_ref().and_then(|v| match v { ValueEnum::Int(i) => Some(*i), _ => None })).collect()),
                "Float" => TinyColumn::OptFloat(value_enum_vals.iter().map(|x| x.as_ref().and_then(|v| match v { ValueEnum::Float(f) => Some(*f), _ => None })).collect()),
                "Bool" => TinyColumn::OptBool(value_enum_vals.iter().map(|x| x.as_ref().and_then(|v| match v { ValueEnum::Bool(b) => Some(*b), _ => None })).collect()),
                "Str" => TinyColumn::OptStr(value_enum_vals.iter().map(|x| x.as_ref().and_then(|v| match v { ValueEnum::Str(s) => Some(s.clone()), _ => None })).collect()),
                "PyObject" => TinyColumn::OptPyObject(value_enum_vals.iter().map(|x| x.as_ref().and_then(|v| match v { ValueEnum::PyObjectId(id) => Some(*id), _ => None })).collect()),
                _ => unreachable!(),
            }
        } else {
            TinyColumn::Mixed(value_enum_vals.into_iter().map(|x| x.unwrap()).collect())
        };

        columns.insert(col.clone(), final_col);
    }

    Ok(TinyFrame { columns, length: num_rows, py_objects })
}

fn id_fallback(obj: &PyAny) -> u64 {
    obj.as_ptr() as u64
}

pub fn to_dicts_impl(frame: &TinyFrame, py: Python) -> PyResult<Vec<PyObject>> {
    let mut result = Vec::new();
    for i in 0..frame.length {
        let dict = PyDict::new(py);
        for (col_name, col_data) in &frame.columns {
            let val = match col_data {
                TinyColumn::Int(v) => v[i].into_py(py),
                TinyColumn::Float(v) => v[i].into_py(py),
                TinyColumn::Bool(v) => v[i].into_py(py),
                TinyColumn::Str(v) => v[i].clone().into_py(py),
                TinyColumn::OptInt(v) => v[i].map_or(py.None(), |x| x.into_py(py)),
                TinyColumn::OptFloat(v) => v[i].map_or(py.None(), |x| x.into_py(py)),
                TinyColumn::OptBool(v) => v[i].map_or(py.None(), |x| x.into_py(py)),
                TinyColumn::OptStr(v) => v[i].clone().map_or(py.None(), |x| x.into_py(py)),
                TinyColumn::PyObject(v) => {
                    let id = v[i];
                    frame.py_objects.get(&id).cloned().unwrap_or(py.None())
                }
                TinyColumn::OptPyObject(v) => match v[i] {
                    Some(id) => frame.py_objects.get(&id).cloned().unwrap_or(py.None()),
                    None => py.None(),
                },
                TinyColumn::Mixed(v) => match &v[i] {
                    ValueEnum::Int(x) => x.into_py(py),
                    ValueEnum::Float(x) => x.into_py(py),
                    ValueEnum::Bool(x) => x.into_py(py),
                    ValueEnum::Str(x) => x.clone().into_py(py),
                    ValueEnum::PyObjectId(id) => frame.py_objects.get(id).cloned().unwrap_or(py.None()),
                },
                TinyColumn::OptMixed(v) => match &v[i] {
                    Some(ValueEnum::Int(x)) => x.into_py(py),
                    Some(ValueEnum::Float(x)) => x.into_py(py),
                    Some(ValueEnum::Bool(x)) => x.into_py(py),
                    Some(ValueEnum::Str(x)) => x.clone().into_py(py),
                    Some(ValueEnum::PyObjectId(id)) => frame.py_objects.get(id).cloned().unwrap_or(py.None()),
                    None => py.None(),
                },
            };
            dict.set_item(col_name, val)?;
        }
        result.push(dict.into());
    }
    Ok(result)
}