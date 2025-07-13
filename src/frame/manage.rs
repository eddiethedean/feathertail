use pyo3::prelude::*;
use crate::frame::TinyFrame;

pub fn drop_columns_impl(frame: &mut TinyFrame, columns_to_drop: Vec<String>) -> PyResult<()> {
    for col_name in columns_to_drop {
        frame.columns.remove(&col_name);
    }
    Ok(())
}

pub fn rename_column_impl(frame: &mut TinyFrame, old_name: String, new_name: String) -> PyResult<()> {
    if !frame.columns.contains_key(&old_name) {
        return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", old_name)));
    }
    if frame.columns.contains_key(&new_name) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{}' already exists", new_name)));
    }
    let col = frame.columns.remove(&old_name).unwrap();
    frame.columns.insert(new_name, col);
    Ok(())
}
