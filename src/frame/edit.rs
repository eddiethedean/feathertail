use pyo3::prelude::*;
use pyo3::PyObject;
use crate::frame::{TinyColumn, ValueEnum};
use crate::utils::{convert_pyobject_to_valueenum, pyobject_to_option_valueenum};

pub fn edit_column_impl(frame: &mut crate::frame::TinyFrame, py: Python, column_name: String, func: PyObject) -> PyResult<()> {
    let col = frame.columns.get_mut(&column_name).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column_name))
    })?;
    edit_column_logic(col, py, func, &mut frame.py_objects)
}

fn edit_column_logic(col: &mut TinyColumn, py: Python, func: PyObject, py_objects: &mut std::collections::HashMap<u64, PyObject>) -> PyResult<()> {
    let len = match col {
        TinyColumn::Int(v) => v.len(),
        TinyColumn::Float(v) => v.len(),
        TinyColumn::Bool(v) => v.len(),
        TinyColumn::Str(v) => v.len(),
        TinyColumn::OptInt(v) => v.len(),
        TinyColumn::OptFloat(v) => v.len(),
        TinyColumn::OptBool(v) => v.len(),
        TinyColumn::OptStr(v) => v.len(),
        TinyColumn::Mixed(v) => v.len(),
        TinyColumn::OptMixed(v) => v.len(),
        TinyColumn::PyObject(v) => v.len(),
        TinyColumn::OptPyObject(v) => v.len(),
    };

    // Prepare a new mixed or pyobject vector to hold updated values
    let mut new_values: Vec<Option<ValueEnum>> = Vec::with_capacity(len);

    for i in 0..len {
        let input_obj = match col {
            TinyColumn::Int(v) => v[i].into_py(py),
            TinyColumn::Float(v) => v[i].into_py(py),
            TinyColumn::Bool(v) => v[i].into_py(py),
            TinyColumn::Str(v) => v[i].clone().into_py(py),
            TinyColumn::OptInt(v) => v[i].map_or_else(|| py.None(), |x| x.into_py(py)),
            TinyColumn::OptFloat(v) => v[i].map_or_else(|| py.None(), |x| x.into_py(py)),
            TinyColumn::OptBool(v) => v[i].map_or_else(|| py.None(), |x| x.into_py(py)),
            TinyColumn::OptStr(v) => v[i].clone().map_or_else(|| py.None(), |x| x.into_py(py)),
            TinyColumn::Mixed(v) => match &v[i] {
                ValueEnum::Int(x) => x.into_py(py),
                ValueEnum::Float(x) => x.into_py(py),
                ValueEnum::Bool(x) => x.into_py(py),
                ValueEnum::Str(x) => x.clone().into_py(py),
                ValueEnum::PyObjectId(id) => py_objects.get(id).unwrap().clone(),
            },
            TinyColumn::OptMixed(v) => match &v[i] {
                Some(ValueEnum::Int(x)) => x.into_py(py),
                Some(ValueEnum::Float(x)) => x.into_py(py),
                Some(ValueEnum::Bool(x)) => x.into_py(py),
                Some(ValueEnum::Str(x)) => x.clone().into_py(py),
                Some(ValueEnum::PyObjectId(id)) => py_objects.get(id).unwrap().clone(),
                None => py.None(),
            },
            TinyColumn::PyObject(v) => py_objects.get(&v[i]).unwrap().clone(),
            TinyColumn::OptPyObject(v) => match v[i] {
                Some(id) => py_objects.get(&id).unwrap().clone(),
                None => py.None(),
            },
        };

        let edited_py_value = func.call1(py, (input_obj,))?;

        // Convert the new value to ValueEnum or fallback PyObject
        let maybe_enum = pyobject_to_option_valueenum(edited_py_value.as_ref(py), py_objects)?;

        new_values.push(maybe_enum);
    }

    // Check if there are any None
    let has_none = new_values.iter().any(|v| v.is_none());

    // Check for presence of fallback PyObject IDs
    let has_pyobjects = new_values.iter().any(|v| {
        matches!(v, Some(ValueEnum::PyObjectId(_)))
    });

    // Choose final column type
    if has_pyobjects {
        if has_none {
            let vec = new_values.into_iter().map(|opt| opt.map(|v| match v {
                ValueEnum::PyObjectId(id) => id,
                _ => panic!("Unexpected non-PyObjectId in OptPyObject"),
            })).collect();
            *col = TinyColumn::OptPyObject(vec);
        } else {
            let vec = new_values.into_iter().map(|opt| match opt {
                Some(ValueEnum::PyObjectId(id)) => id,
                _ => panic!("Unexpected non-PyObjectId in PyObject"),
            }).collect();
            *col = TinyColumn::PyObject(vec);
        }
    } else {
        // Standard Rust type fallback: Mixed or OptMixed
        if has_none {
            *col = TinyColumn::OptMixed(new_values);
        } else {
            let no_opt: Vec<ValueEnum> = new_values.into_iter().map(|v| v.unwrap()).collect();
            *col = TinyColumn::Mixed(no_opt);
        }
    }

    Ok(())
}
