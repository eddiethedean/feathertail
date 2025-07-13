use pyo3::prelude::*;
use pyo3::PyObject;
use crate::frame::{TinyFrame, TinyColumn};

pub fn edit_column_impl(frame: &mut TinyFrame, py: Python, column_name: String, func: PyObject) -> PyResult<()> {
    let col = frame.columns.get_mut(&column_name).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column_name))
    })?;
    edit_column_logic(col, py, func)
}

fn edit_column_logic(col: &mut TinyColumn, py: Python, func: PyObject) -> PyResult<()> {
    match col {
        TinyColumn::Str(vec) => {
            for v in vec.iter_mut() {
                let input = v.clone().into_py(py);
                let out_obj = func.call1(py, (input,))?;
                *v = out_obj.extract(py)?;
            }
        }
        TinyColumn::Int(vec) => {
            for v in vec.iter_mut() {
                let input = (*v).into_py(py);
                let out_obj = func.call1(py, (input,))?;
                *v = out_obj.extract(py)?;
            }
        }
        TinyColumn::Float(vec) => {
            for v in vec.iter_mut() {
                let input = (*v).into_py(py);
                let out_obj = func.call1(py, (input,))?;
                *v = out_obj.extract(py)?;
            }
        }
        TinyColumn::Bool(vec) => {
            for v in vec.iter_mut() {
                let input = (*v).into_py(py);
                let out_obj = func.call1(py, (input,))?;
                *v = out_obj.extract(py)?;
            }
        }
        TinyColumn::OptStr(vec) => {
            for v in vec.iter_mut() {
                let input = v.clone().map_or_else(|| py.None(), |s| s.into_py(py));
                let out_obj = func.call1(py, (input,))?;
                *v = if out_obj.is_none(py) { None } else { Some(out_obj.extract(py)?) };
            }
        }
        TinyColumn::OptInt(vec) => {
            for v in vec.iter_mut() {
                let input = v.map_or_else(|| py.None(), |i| i.into_py(py));
                let out_obj = func.call1(py, (input,))?;
                *v = if out_obj.is_none(py) { None } else { Some(out_obj.extract(py)?) };
            }
        }
        TinyColumn::OptFloat(vec) => {
            for v in vec.iter_mut() {
                let input = v.map_or_else(|| py.None(), |f| f.into_py(py));
                let out_obj = func.call1(py, (input,))?;
                *v = if out_obj.is_none(py) { None } else { Some(out_obj.extract(py)?) };
            }
        }
        TinyColumn::OptBool(vec) => {
            for v in vec.iter_mut() {
                let input = v.map_or_else(|| py.None(), |b| b.into_py(py));
                let out_obj = func.call1(py, (input,))?;
                *v = if out_obj.is_none(py) { None } else { Some(out_obj.extract(py)?) };
            }
        }
        TinyColumn::Mixed(_) | TinyColumn::OptMixed(_) => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "edit_column is not supported for Mixed and OptMixed columns yet",
            ));
        }
    }
    Ok(())
}
