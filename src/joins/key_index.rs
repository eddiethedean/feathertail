//! Build join key → row index multimap from frame columns.

use std::collections::HashMap;
use pyo3::prelude::*;
use crate::frame::{TinyColumn, TinyFrame, ValueEnum};

pub fn build_key_map(frame: &TinyFrame, columns: &[String]) -> PyResult<HashMap<Vec<ValueEnum>, Vec<usize>>> {
    let mut key_map: HashMap<Vec<ValueEnum>, Vec<usize>> = HashMap::new();

    let n_key_cols = columns.len();
    if n_key_cols == 0 {
        return Ok(key_map);
    }

    'row: for row_idx in 0..frame.length {
        let mut key = Vec::with_capacity(n_key_cols);
        for col_name in columns {
            let col = frame.columns.get(col_name).unwrap();
            match join_key_value_at(col, row_idx) {
                Some(value) => key.push(value),
                None => {
                    continue 'row;
                }
            }
        }
        key_map.entry(key).or_insert_with(Vec::new).push(row_idx);
    }

    Ok(key_map)
}

pub fn join_key_value_at(col: &TinyColumn, index: usize) -> Option<ValueEnum> {
    match col {
        TinyColumn::Int(v) => v.get(index).map(|&val| ValueEnum::Int(val)),
        TinyColumn::Float(v) => v.get(index).map(|&val| ValueEnum::Float(val)),
        TinyColumn::Str(v) => v.get(index).map(|val| ValueEnum::Str(val.clone())),
        TinyColumn::Bool(v) => v.get(index).map(|&val| ValueEnum::Bool(val)),
        TinyColumn::PyObject(v) => v.get(index).map(|&val| ValueEnum::PyObjectId(val)),
        TinyColumn::Mixed(v) => v.get(index).map(|val| val.clone()),
        TinyColumn::OptInt(v) => v.get(index).and_then(|val| val.map(ValueEnum::Int)),
        TinyColumn::OptFloat(v) => v.get(index).and_then(|val| val.map(ValueEnum::Float)),
        TinyColumn::OptStr(v) => v
            .get(index)
            .and_then(|val| val.as_ref().map(|s| ValueEnum::Str(s.clone()))),
        TinyColumn::OptBool(v) => v.get(index).and_then(|val| val.map(ValueEnum::Bool)),
        TinyColumn::OptPyObject(v) => v.get(index).and_then(|val| val.map(ValueEnum::PyObjectId)),
        TinyColumn::OptMixed(v) => v.get(index).and_then(|val| val.clone()),
    }
}
