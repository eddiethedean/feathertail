use crate::frame::{TinyColumn, TinyFrame, ValueEnum};
use pyo3::prelude::*;

pub fn stringify_column(col: &TinyColumn, idx: usize) -> String {
    match col {
        TinyColumn::Int(v) => v[idx].to_string(),
        TinyColumn::Float(v) => v[idx].to_string(),
        TinyColumn::Bool(v) => v[idx].to_string(),
        TinyColumn::Str(v) => v[idx].clone(),
        TinyColumn::OptInt(v) => v[idx].map(|x| x.to_string()).unwrap_or_else(|| "None".to_string()),
        TinyColumn::OptFloat(v) => v[idx].map(|x| x.to_string()).unwrap_or_else(|| "None".to_string()),
        TinyColumn::OptBool(v) => v[idx].map(|x| x.to_string()).unwrap_or_else(|| "None".to_string()),
        TinyColumn::OptStr(v) => v[idx].clone().unwrap_or_else(|| "None".to_string()),
        TinyColumn::Mixed(v) => match &v[idx] {
            ValueEnum::Int(x) => x.to_string(),
            ValueEnum::Float(x) => x.to_string(),
            ValueEnum::Bool(x) => x.to_string(),
            ValueEnum::Str(x) => x.clone(),
        },
        TinyColumn::OptMixed(v) => match &v[idx] {
            Some(ValueEnum::Int(x)) => x.to_string(),
            Some(ValueEnum::Float(x)) => x.to_string(),
            Some(ValueEnum::Bool(x)) => x.to_string(),
            Some(ValueEnum::Str(x)) => x.clone(),
            None => "None".to_string(),
        },
    }
}
