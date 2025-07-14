use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};

pub fn fillna_impl(frame: &mut TinyFrame, py: Python, value: &PyAny) -> PyResult<()> {
    let is_dict = value.is_instance_of::<pyo3::types::PyDict>();
    if is_dict {
        let dict = value.downcast::<pyo3::types::PyDict>()?;
        for (col_name_py, val_obj) in dict {
            let col_name: String = col_name_py.extract()?;
            let col = frame.columns.get_mut(&col_name).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", col_name))
            })?;
            fill_column(col, val_obj)?;
            convert_if_fully_filled(col);
        }
    } else {
        for (_, col) in frame.columns.iter_mut() {
            fill_column(col, value)?;
            convert_if_fully_filled(col);
        }
    }
    Ok(())
}

fn fill_column(col: &mut TinyColumn, value: &PyAny) -> PyResult<()> {
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
            let inferred = if value.is_instance_of::<pyo3::types::PyBool>() {
                ValueEnum::Bool(value.extract()?)
            } else if value.is_instance_of::<pyo3::types::PyLong>() {
                ValueEnum::Int(value.extract()?)
            } else if value.is_instance_of::<pyo3::types::PyFloat>() {
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
            // Non-optional columns do not need fillna
        }
    }
    Ok(())
}

/// Convert optional columns to non-optional if no None remains
fn convert_if_fully_filled(col: &mut TinyColumn) {
    match col {
        TinyColumn::OptInt(vec) if vec.iter().all(|x| x.is_some()) => {
            let new_vec = vec.iter().map(|x| x.unwrap()).collect();
            *col = TinyColumn::Int(new_vec);
        }
        TinyColumn::OptFloat(vec) if vec.iter().all(|x| x.is_some()) => {
            let new_vec = vec.iter().map(|x| x.unwrap()).collect();
            *col = TinyColumn::Float(new_vec);
        }
        TinyColumn::OptBool(vec) if vec.iter().all(|x| x.is_some()) => {
            let new_vec = vec.iter().map(|x| x.unwrap()).collect();
            *col = TinyColumn::Bool(new_vec);
        }
        TinyColumn::OptStr(vec) if vec.iter().all(|x| x.is_some()) => {
            let new_vec = vec.iter().map(|x| x.clone().unwrap()).collect();
            *col = TinyColumn::Str(new_vec);
        }
        TinyColumn::OptMixed(vec) if vec.iter().all(|x| x.is_some()) => {
            let new_vec = vec.iter().map(|x| x.clone().unwrap()).collect();
            *col = TinyColumn::Mixed(new_vec);
        }
        _ => {}
    }
}
