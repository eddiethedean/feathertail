//! Typed column storage for `TinyFrame` (`TinyColumn`).

use super::value::ValueEnum;
use pyo3::prelude::*;

#[derive(Clone)]
pub enum TinyColumn {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Str(Vec<String>),
    Bool(Vec<bool>),
    OptInt(Vec<Option<i64>>),
    OptFloat(Vec<Option<f64>>),
    OptStr(Vec<Option<String>>),
    OptBool(Vec<Option<bool>>),
    Mixed(Vec<ValueEnum>),
    OptMixed(Vec<Option<ValueEnum>>),
    PyObject(Vec<u64>),
    OptPyObject(Vec<Option<u64>>),
}

impl TinyColumn {
    /// Short name used by Python-facing `TinyCol.type_str`.
    pub fn type_tag(&self) -> &'static str {
        match self {
            TinyColumn::Int(_) => "Int",
            TinyColumn::Float(_) => "Float",
            TinyColumn::Str(_) => "Str",
            TinyColumn::Bool(_) => "Bool",
            TinyColumn::OptInt(_) => "OptInt",
            TinyColumn::OptFloat(_) => "OptFloat",
            TinyColumn::OptStr(_) => "OptStr",
            TinyColumn::OptBool(_) => "OptBool",
            TinyColumn::Mixed(_) => "Mixed",
            TinyColumn::OptMixed(_) => "OptMixed",
            TinyColumn::PyObject(_) => "PyObject",
            TinyColumn::OptPyObject(_) => "OptPyObject",
        }
    }

    /// Empty column with the same storage variant as `self` (non-optional layout).
    pub fn empty_same_layout(&self) -> Self {
        match self {
            TinyColumn::Int(_) => TinyColumn::Int(Vec::new()),
            TinyColumn::Float(_) => TinyColumn::Float(Vec::new()),
            TinyColumn::Str(_) => TinyColumn::Str(Vec::new()),
            TinyColumn::Bool(_) => TinyColumn::Bool(Vec::new()),
            TinyColumn::PyObject(_) => TinyColumn::PyObject(Vec::new()),
            TinyColumn::Mixed(_) => TinyColumn::Mixed(Vec::new()),
            TinyColumn::OptInt(_) => TinyColumn::OptInt(Vec::new()),
            TinyColumn::OptFloat(_) => TinyColumn::OptFloat(Vec::new()),
            TinyColumn::OptStr(_) => TinyColumn::OptStr(Vec::new()),
            TinyColumn::OptBool(_) => TinyColumn::OptBool(Vec::new()),
            TinyColumn::OptPyObject(_) => TinyColumn::OptPyObject(Vec::new()),
            TinyColumn::OptMixed(_) => TinyColumn::OptMixed(Vec::new()),
        }
    }

    /// Empty column using optional storage where the prototype is non-optional
    /// (for join / outer-null paths); otherwise matches the prototype layout.
    pub fn empty_optional_layout(&self) -> Self {
        match self {
            TinyColumn::Int(_) => TinyColumn::OptInt(Vec::new()),
            TinyColumn::Float(_) => TinyColumn::OptFloat(Vec::new()),
            TinyColumn::Str(_) => TinyColumn::OptStr(Vec::new()),
            TinyColumn::Bool(_) => TinyColumn::OptBool(Vec::new()),
            TinyColumn::PyObject(_) => TinyColumn::OptPyObject(Vec::new()),
            TinyColumn::Mixed(_) => TinyColumn::OptMixed(Vec::new()),
            TinyColumn::OptInt(_) => TinyColumn::OptInt(Vec::new()),
            TinyColumn::OptFloat(_) => TinyColumn::OptFloat(Vec::new()),
            TinyColumn::OptStr(_) => TinyColumn::OptStr(Vec::new()),
            TinyColumn::OptBool(_) => TinyColumn::OptBool(Vec::new()),
            TinyColumn::OptPyObject(_) => TinyColumn::OptPyObject(Vec::new()),
            TinyColumn::OptMixed(_) => TinyColumn::OptMixed(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            TinyColumn::Int(v) => v.len(),
            TinyColumn::Float(v) => v.len(),
            TinyColumn::Str(v) => v.len(),
            TinyColumn::Bool(v) => v.len(),
            TinyColumn::OptInt(v) => v.len(),
            TinyColumn::OptFloat(v) => v.len(),
            TinyColumn::OptStr(v) => v.len(),
            TinyColumn::OptBool(v) => v.len(),
            TinyColumn::Mixed(v) => v.len(),
            TinyColumn::OptMixed(v) => v.len(),
            TinyColumn::PyObject(v) => v.len(),
            TinyColumn::OptPyObject(v) => v.len(),
        }
    }

    pub fn iter(&self) -> TinyColumnIter<'_> {
        TinyColumnIter::new(self)
    }

    /// Append one row from `src` at `idx` when destination and source types match exactly
    /// (legacy `utils::append_value` behavior).
    pub fn append_row_strict(&mut self, src: &TinyColumn, idx: usize) -> Result<(), ()> {
        match (self, src) {
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
            _ => return Err(()),
        }
        Ok(())
    }

    /// Join row append: allows optional columns to receive non-optional sources.
    pub fn append_row_join(&mut self, src: &TinyColumn, source_idx: usize) -> PyResult<()> {
        match (self, src) {
            (TinyColumn::Int(target), TinyColumn::Int(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Float(target), TinyColumn::Float(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Str(target), TinyColumn::Str(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::Bool(target), TinyColumn::Bool(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::PyObject(target), TinyColumn::PyObject(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Mixed(target), TinyColumn::Mixed(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::OptInt(target), TinyColumn::OptInt(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptFloat(target), TinyColumn::OptFloat(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptStr(target), TinyColumn::OptStr(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::OptBool(target), TinyColumn::OptBool(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptPyObject(target), TinyColumn::OptPyObject(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptMixed(target), TinyColumn::OptMixed(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::OptInt(target), TinyColumn::Int(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptFloat(target), TinyColumn::Float(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptStr(target), TinyColumn::Str(source)) => {
                target.push(Some(source[source_idx].clone()));
            }
            (TinyColumn::OptBool(target), TinyColumn::Bool(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptPyObject(target), TinyColumn::PyObject(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptMixed(target), TinyColumn::Mixed(source)) => {
                target.push(Some(source[source_idx].clone()));
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Column type mismatch in join operation",
                ));
            }
        }
        Ok(())
    }

    /// Append a null row, promoting non-optional columns to optional storage when needed.
    pub fn push_null_or_promote(&mut self) -> PyResult<()> {
        match self {
            TinyColumn::OptInt(target) => {
                target.push(None);
            }
            TinyColumn::OptFloat(target) => {
                target.push(None);
            }
            TinyColumn::OptStr(target) => {
                target.push(None);
            }
            TinyColumn::OptBool(target) => {
                target.push(None);
            }
            TinyColumn::OptPyObject(target) => {
                target.push(None);
            }
            TinyColumn::OptMixed(target) => {
                target.push(None);
            }
            TinyColumn::Int(target) => {
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptInt(opt_vec);
            }
            TinyColumn::Float(target) => {
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptFloat(opt_vec);
            }
            TinyColumn::Str(target) => {
                let mut opt_vec = Vec::new();
                for val in target.iter() {
                    opt_vec.push(Some(val.clone()));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptStr(opt_vec);
            }
            TinyColumn::Bool(target) => {
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptBool(opt_vec);
            }
            TinyColumn::PyObject(target) => {
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptPyObject(opt_vec);
            }
            TinyColumn::Mixed(target) => {
                let mut opt_vec = Vec::new();
                for val in target.iter() {
                    opt_vec.push(Some(val.clone()));
                }
                opt_vec.push(None);
                *self = TinyColumn::OptMixed(opt_vec);
            }
        }
        Ok(())
    }

    /// Contiguous row slice `[start, end)` (used by chunked loaders).
    pub fn slice_row_range(&self, start: usize, end: usize) -> Self {
        match self {
            TinyColumn::Int(v) => TinyColumn::Int(v[start..end].to_vec()),
            TinyColumn::Float(v) => TinyColumn::Float(v[start..end].to_vec()),
            TinyColumn::Str(v) => TinyColumn::Str(v[start..end].to_vec()),
            TinyColumn::Bool(v) => TinyColumn::Bool(v[start..end].to_vec()),
            TinyColumn::PyObject(v) => TinyColumn::PyObject(v[start..end].to_vec()),
            TinyColumn::Mixed(v) => TinyColumn::Mixed(v[start..end].to_vec()),
            TinyColumn::OptInt(v) => TinyColumn::OptInt(v[start..end].to_vec()),
            TinyColumn::OptFloat(v) => TinyColumn::OptFloat(v[start..end].to_vec()),
            TinyColumn::OptStr(v) => TinyColumn::OptStr(v[start..end].to_vec()),
            TinyColumn::OptBool(v) => TinyColumn::OptBool(v[start..end].to_vec()),
            TinyColumn::OptPyObject(v) => TinyColumn::OptPyObject(v[start..end].to_vec()),
            TinyColumn::OptMixed(v) => TinyColumn::OptMixed(v[start..end].to_vec()),
        }
    }

    /// Rows selected by `indices` (used by filtering / permutation paths).
    pub fn gather_rows(&self, indices: &[usize]) -> Self {
        match self {
            TinyColumn::Int(v) => TinyColumn::Int(indices.iter().map(|&i| v[i]).collect()),
            TinyColumn::Float(v) => TinyColumn::Float(indices.iter().map(|&i| v[i]).collect()),
            TinyColumn::Str(v) => TinyColumn::Str(indices.iter().map(|&i| v[i].clone()).collect()),
            TinyColumn::Bool(v) => TinyColumn::Bool(indices.iter().map(|&i| v[i]).collect()),
            TinyColumn::PyObject(v) => {
                TinyColumn::PyObject(indices.iter().map(|&i| v[i]).collect())
            }
            TinyColumn::Mixed(v) => {
                TinyColumn::Mixed(indices.iter().map(|&i| v[i].clone()).collect())
            }
            TinyColumn::OptInt(v) => TinyColumn::OptInt(indices.iter().map(|&i| v[i]).collect()),
            TinyColumn::OptFloat(v) => {
                TinyColumn::OptFloat(indices.iter().map(|&i| v[i]).collect())
            }
            TinyColumn::OptStr(v) => {
                TinyColumn::OptStr(indices.iter().map(|&i| v[i].clone()).collect())
            }
            TinyColumn::OptBool(v) => TinyColumn::OptBool(indices.iter().map(|&i| v[i]).collect()),
            TinyColumn::OptPyObject(v) => {
                TinyColumn::OptPyObject(indices.iter().map(|&i| v[i]).collect())
            }
            TinyColumn::OptMixed(v) => {
                TinyColumn::OptMixed(indices.iter().map(|&i| v[i].clone()).collect())
            }
        }
    }
}

pub struct TinyColumnIter<'a> {
    column: &'a TinyColumn,
    index: usize,
}

impl<'a> TinyColumnIter<'a> {
    fn new(column: &'a TinyColumn) -> Self {
        TinyColumnIter { column, index: 0 }
    }
}

impl<'a> Iterator for TinyColumnIter<'a> {
    type Item = ValueEnum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.column.len() {
            return None;
        }

        let value = match self.column {
            TinyColumn::Int(v) => ValueEnum::Int(v[self.index]),
            TinyColumn::Float(v) => ValueEnum::Float(v[self.index]),
            TinyColumn::Str(v) => ValueEnum::Str(v[self.index].clone()),
            TinyColumn::Bool(v) => ValueEnum::Bool(v[self.index]),
            TinyColumn::OptInt(v) => {
                self.index += 1;
                return v[self.index - 1].map(ValueEnum::Int);
            }
            TinyColumn::OptFloat(v) => {
                self.index += 1;
                return v[self.index - 1].map(ValueEnum::Float);
            }
            TinyColumn::OptStr(v) => {
                self.index += 1;
                return v[self.index - 1].clone().map(ValueEnum::Str);
            }
            TinyColumn::OptBool(v) => {
                self.index += 1;
                return v[self.index - 1].map(ValueEnum::Bool);
            }
            TinyColumn::Mixed(v) => v[self.index].clone(),
            TinyColumn::OptMixed(v) => {
                self.index += 1;
                return v[self.index - 1].clone();
            }
            TinyColumn::PyObject(v) => ValueEnum::PyObjectId(v[self.index]),
            TinyColumn::OptPyObject(v) => {
                self.index += 1;
                return v[self.index - 1].map(ValueEnum::PyObjectId);
            }
        };

        self.index += 1;
        Some(value)
    }
}
