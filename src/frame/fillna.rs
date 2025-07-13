use pyo3::prelude::*;
use crate::frame::{TinyColumn, ValueEnum};

pub fn fillna_impl(frame: &mut crate::frame::TinyFrame, py: Python, value: &PyAny) -> PyResult<()> {
    let is_dict = value.is_instance_of::<pyo3::types::PyDict>();
    if is_dict {
        let dict = value.downcast::<pyo3::types::PyDict>()?;
        for (col_name_py, val_obj) in dict {
            let col_name: String = col_name_py.extract()?;
            let col = frame.columns.get_mut(&col_name).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", col_name))
            })?;
            fill_column(col, val_obj, py)?;
        }
    } else {
        for (_, col) in frame.columns.iter_mut() {
            fill_column(col, value, py)?;
        }
    }
    Ok(())
}

fn fill_column(col: &mut TinyColumn, value: &PyAny, py: Python) -> PyResult<()> {
    match col {
        TinyColumn::OptStr(vec) => {
            let val: String = value.extract()?;
            for v in vec.iter_mut() {
                if v.is_none() {
                    *v = Some(val.clone());
                }
            }
        }
        TinyColumn::OptInt(vec) => {
            let val: i64 = value.extract()?;
            for v in vec.iter_mut() {
                if v.is_none() {
                    *v = Some(val);
                }
            }
        }
        TinyColumn::OptFloat(vec) => {
            let val: f64 = value.extract()?;
            for v in vec.iter_mut() {
                if v.is_none() {
                    *v = Some(val);
                }
            }
        }
        TinyColumn::OptBool(vec) => {
            let val: bool = value.extract()?;
            for v in vec.iter_mut() {
                if v.is_none() {
                    *v = Some(val);
                }
            }
        }
        TinyColumn::OptMixed(vec) => {
            let inferred: ValueEnum = if value.is_instance(py.get_type::<pyo3::types::PyBool>())? {
                ValueEnum::Bool(value.extract()?)
            } else if value.is_instance(py.get_type::<pyo3::types::PyLong>())? {
                ValueEnum::Int(value.extract()?)
            } else if value.is_instance(py.get_type::<pyo3::types::PyFloat>())? {
                ValueEnum::Float(value.extract()?)
            } else {
                ValueEnum::Str(value.extract()?)
            };
            for v in vec.iter_mut() {
                if v.is_none() {
                    *v = Some(inferred.clone());
                }
            }
        }
        _ => {
            // Non-optional columns don't need fill
        }
    }
    Ok(())
}
