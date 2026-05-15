//! Scalar cell values carried in mixed/object columns (`ValueEnum`).

use pyo3::prelude::*;
use pyo3::IntoPy;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueEnum {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    PyObjectId(u64),
}

impl std::hash::Hash for ValueEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ValueEnum::Int(i) => i.hash(state),
            ValueEnum::Float(f) => {
                if f.is_nan() {
                    state.write_u64(0x7ff8000000000000u64);
                } else if f.is_infinite() {
                    if f.is_sign_positive() {
                        state.write_u64(0x7ff0000000000000u64);
                    } else {
                        state.write_u64(0xfff0000000000000u64);
                    }
                } else {
                    f.to_bits().hash(state);
                }
            }
            ValueEnum::Str(s) => s.hash(state),
            ValueEnum::Bool(b) => b.hash(state),
            ValueEnum::PyObjectId(id) => id.hash(state),
        }
    }
}

impl std::cmp::Eq for ValueEnum {}

impl std::cmp::Ord for ValueEnum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ValueEnum::Int(a), ValueEnum::Int(b)) => a.cmp(b),
            (ValueEnum::Float(a), ValueEnum::Float(b)) => {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            }
            (ValueEnum::Str(a), ValueEnum::Str(b)) => a.cmp(b),
            (ValueEnum::Bool(a), ValueEnum::Bool(b)) => a.cmp(b),
            (ValueEnum::PyObjectId(a), ValueEnum::PyObjectId(b)) => a.cmp(b),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl std::cmp::PartialOrd for ValueEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ValueEnum {
    pub fn to_py(&self, py: Python, py_objects: &HashMap<u64, PyObject>) -> PyObject {
        match self {
            ValueEnum::Int(v) => v.into_py(py),
            ValueEnum::Float(v) => v.into_py(py),
            ValueEnum::Str(v) => v.clone().into_py(py),
            ValueEnum::Bool(v) => v.into_py(py),
            ValueEnum::PyObjectId(id) => py_objects
                .get(id)
                .map(|o| o.clone_ref(py))
                .unwrap_or_else(|| py.None()),
        }
    }
}
