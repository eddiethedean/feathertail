use crate::frame::{TinyColumn, ValueEnum};

pub fn empty_like_column(col: &TinyColumn) -> TinyColumn {
    match col {
        TinyColumn::Int(_) => TinyColumn::Int(Vec::new()),
        TinyColumn::Float(_) => TinyColumn::Float(Vec::new()),
        TinyColumn::Bool(_) => TinyColumn::Bool(Vec::new()),
        TinyColumn::Str(_) => TinyColumn::Str(Vec::new()),
        TinyColumn::OptInt(_) => TinyColumn::OptInt(Vec::new()),
        TinyColumn::OptFloat(_) => TinyColumn::OptFloat(Vec::new()),
        TinyColumn::OptBool(_) => TinyColumn::OptBool(Vec::new()),
        TinyColumn::OptStr(_) => TinyColumn::OptStr(Vec::new()),
        TinyColumn::Mixed(_) => TinyColumn::Mixed(Vec::new()),
        TinyColumn::OptMixed(_) => TinyColumn::OptMixed(Vec::new()),
        TinyColumn::PyObject(_) => TinyColumn::PyObject(Vec::new()),
        TinyColumn::OptPyObject(_) => TinyColumn::OptPyObject(Vec::new()),
    }
}

pub fn append_value(col: &mut TinyColumn, idx: usize, src: &TinyColumn) {
    match (col, src) {
        (TinyColumn::Int(dst), TinyColumn::Int(src)) => dst.push(src[idx]),
        (TinyColumn::Float(dst), TinyColumn::Float(src)) => dst.push(src[idx]),
        (TinyColumn::Bool(dst), TinyColumn::Bool(src)) => dst.push(src[idx]),
        (TinyColumn::Str(dst), TinyColumn::Str(src)) => dst.push(src[idx].clone()),
        (TinyColumn::OptInt(dst), TinyColumn::OptInt(src)) => dst.push(src[idx]),
        (TinyColumn::OptFloat(dst), TinyColumn::OptFloat(src)) => dst.push(src[idx]),
        (TinyColumn::OptBool(dst), TinyColumn::OptBool(src)) => dst.push(src[idx]),
        (TinyColumn::OptStr(dst), TinyColumn::OptStr(src)) => dst.push(src[idx].clone()),
        (TinyColumn::Mixed(dst), TinyColumn::Mixed(src)) => dst.push(src[idx].clone()),
        (TinyColumn::OptMixed(dst), TinyColumn::OptMixed(src)) => dst.push(src[idx].clone()),
        (TinyColumn::PyObject(dst), TinyColumn::PyObject(src)) => dst.push(src[idx]),
        (TinyColumn::OptPyObject(dst), TinyColumn::OptPyObject(src)) => dst.push(src[idx]),
        _ => panic!("Column type mismatch in append_value"),
    }
}
