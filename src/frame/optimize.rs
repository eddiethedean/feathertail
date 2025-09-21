use pyo3::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};

// Copy-on-write wrapper for columns
#[derive(Clone)]
pub struct CowColumn<'a> {
    inner: Cow<'a, TinyColumn>,
}

impl<'a> CowColumn<'a> {
    pub fn new(column: &'a TinyColumn) -> Self {
        Self {
            inner: Cow::Borrowed(column),
        }
    }

    pub fn to_owned(self) -> TinyColumn {
        self.inner.into_owned()
    }

    pub fn get_value(&self, index: usize) -> Option<ValueEnum> {
        match &self.inner {
            Cow::Borrowed(col) => self.get_value_ref(col, index),
            Cow::Owned(col) => self.get_value_ref(col, index),
        }
    }

    fn get_value_ref(&self, col: &TinyColumn, index: usize) -> Option<ValueEnum> {
        if index >= col.len() {
            return None;
        }

        match col {
            TinyColumn::Int(v) => Some(ValueEnum::Int(v[index])),
            TinyColumn::Float(v) => Some(ValueEnum::Float(v[index])),
            TinyColumn::Str(v) => Some(ValueEnum::Str(v[index].clone())),
            TinyColumn::Bool(v) => Some(ValueEnum::Bool(v[index])),
            TinyColumn::OptInt(v) => v[index].map(ValueEnum::Int),
            TinyColumn::OptFloat(v) => v[index].map(ValueEnum::Float),
            TinyColumn::OptStr(v) => v[index].clone().map(ValueEnum::Str),
            TinyColumn::OptBool(v) => v[index].map(ValueEnum::Bool),
            TinyColumn::PyObject(v) => Some(ValueEnum::PyObjectId(v[index])),
            TinyColumn::OptPyObject(v) => v[index].map(ValueEnum::PyObjectId),
            TinyColumn::Mixed(v) => Some(v[index].clone()),
            TinyColumn::OptMixed(v) => v[index].clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

// Filter condition for optimized operations
#[derive(Clone)]
pub struct FilterCondition {
    pub column: String,
    pub condition: String,
    pub value: PyObject,
}

impl FilterCondition {
    pub fn new(column: String, condition: String, value: PyObject) -> Self {
        Self {
            column,
            condition,
            value,
        }
    }

    pub fn evaluate(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match self.condition.as_str() {
            "==" => self.compare_equals(py, val),
            "!=" => self.compare_not_equals(py, val),
            ">" => self.compare_greater(py, val),
            "<" => self.compare_less(py, val),
            ">=" => self.compare_greater_equal(py, val),
            "<=" => self.compare_less_equal(py, val),
            "in" => self.check_in(py, val),
            "not_in" => Ok(!self.check_in(py, val)?),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown condition: {}", self.condition)
            )),
        }
    }

    fn compare_equals(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match val {
            ValueEnum::Str(s) => {
                let py_str = self.value.extract::<String>(py)?;
                Ok(s == &py_str)
            },
            ValueEnum::Int(i) => {
                let py_int = self.value.extract::<i64>(py)?;
                Ok(*i == py_int)
            },
            ValueEnum::Float(f) => {
                let py_float = self.value.extract::<f64>(py)?;
                Ok((*f - py_float).abs() < f64::EPSILON)
            },
            ValueEnum::Bool(b) => {
                let py_bool = self.value.extract::<bool>(py)?;
                Ok(*b == py_bool)
            },
            _ => Ok(false),
        }
    }

    fn compare_not_equals(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        Ok(!self.compare_equals(py, val)?)
    }

    fn compare_greater(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match val {
            ValueEnum::Int(i) => {
                let py_int = self.value.extract::<i64>(py)?;
                Ok(*i > py_int)
            },
            ValueEnum::Float(f) => {
                let py_float = self.value.extract::<f64>(py)?;
                Ok(*f > py_float)
            },
            _ => Ok(false),
        }
    }

    fn compare_less(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match val {
            ValueEnum::Int(i) => {
                let py_int = self.value.extract::<i64>(py)?;
                Ok(*i < py_int)
            },
            ValueEnum::Float(f) => {
                let py_float = self.value.extract::<f64>(py)?;
                Ok(*f < py_float)
            },
            _ => Ok(false),
        }
    }

    fn compare_greater_equal(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match val {
            ValueEnum::Int(i) => {
                let py_int = self.value.extract::<i64>(py)?;
                Ok(*i >= py_int)
            },
            ValueEnum::Float(f) => {
                let py_float = self.value.extract::<f64>(py)?;
                Ok(*f >= py_float)
            },
            _ => Ok(false),
        }
    }

    fn compare_less_equal(&self, py: Python, val: &ValueEnum) -> PyResult<bool> {
        match val {
            ValueEnum::Int(i) => {
                let py_int = self.value.extract::<i64>(py)?;
                Ok(*i <= py_int)
            },
            ValueEnum::Float(f) => {
                let py_float = self.value.extract::<f64>(py)?;
                Ok(*f <= py_float)
            },
            _ => Ok(false),
        }
    }

    fn check_in(&self, _py: Python, _val: &ValueEnum) -> PyResult<bool> {
        // For now, implement a simple version
        // In a full implementation, this would handle list membership
        Ok(false)
    }
}

// Optimized frame operations using references
impl TinyFrame {
    pub fn filter_optimized(&self, py: Python, condition: &FilterCondition) -> PyResult<TinyFrame> {
        let col = self.columns.get(&condition.column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", condition.column)
            ))?;

        // Use iterator instead of manual indexing for better performance
        let mask: Vec<bool> = (0..self.length)
            .map(|i| {
                let value = self.get_value_at_index_optimized(col, i);
                match value {
                    Some(val) => condition.evaluate(py, &val).unwrap_or(false),
                    None => false,
                }
            })
            .collect();

        self.apply_mask_optimized(&mask)
    }

    fn get_value_at_index_optimized(&self, col: &TinyColumn, index: usize) -> Option<ValueEnum> {
        if index >= col.len() {
            return None;
        }

        match col {
            TinyColumn::Int(v) => Some(ValueEnum::Int(v[index])),
            TinyColumn::Float(v) => Some(ValueEnum::Float(v[index])),
            TinyColumn::Str(v) => Some(ValueEnum::Str(v[index].clone())),
            TinyColumn::Bool(v) => Some(ValueEnum::Bool(v[index])),
            TinyColumn::OptInt(v) => v[index].map(ValueEnum::Int),
            TinyColumn::OptFloat(v) => v[index].map(ValueEnum::Float),
            TinyColumn::OptStr(v) => v[index].clone().map(ValueEnum::Str),
            TinyColumn::OptBool(v) => v[index].map(ValueEnum::Bool),
            TinyColumn::PyObject(v) => Some(ValueEnum::PyObjectId(v[index])),
            TinyColumn::OptPyObject(v) => v[index].map(ValueEnum::PyObjectId),
            TinyColumn::Mixed(v) => Some(v[index].clone()),
            TinyColumn::OptMixed(v) => v[index].clone(),
        }
    }

    fn apply_mask_optimized(&self, mask: &[bool]) -> PyResult<TinyFrame> {
        let mut new_columns: HashMap<String, TinyColumn> = HashMap::new();
        
        for (col_name, col_data) in &self.columns {
            let new_col = self.filter_column_optimized(col_data, mask)?;
            new_columns.insert(col_name.clone(), new_col);
        }

        let new_length = mask.iter().filter(|&&x| x).count();

        Ok(TinyFrame {
            columns: new_columns,
            length: new_length,
            py_objects: self.py_objects.clone(),
        })
    }

    fn filter_column_optimized(&self, col: &TinyColumn, mask: &[bool]) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(v) => {
                let new_v: Vec<i64> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Int(new_v))
            },
            TinyColumn::Float(v) => {
                let new_v: Vec<f64> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Float(new_v))
            },
            TinyColumn::Str(v) => {
                let new_v: Vec<String> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| val.clone())
                    .collect();
                Ok(TinyColumn::Str(new_v))
            },
            TinyColumn::Bool(v) => {
                let new_v: Vec<bool> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Bool(new_v))
            },
            TinyColumn::OptInt(v) => {
                let new_v: Vec<Option<i64>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::OptInt(new_v))
            },
            TinyColumn::OptFloat(v) => {
                let new_v: Vec<Option<f64>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::OptFloat(new_v))
            },
            TinyColumn::OptStr(v) => {
                let new_v: Vec<Option<String>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| val.clone())
                    .collect();
                Ok(TinyColumn::OptStr(new_v))
            },
            TinyColumn::OptBool(v) => {
                let new_v: Vec<Option<bool>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::OptBool(new_v))
            },
            TinyColumn::PyObject(v) => {
                let new_v: Vec<u64> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::PyObject(new_v))
            },
            TinyColumn::OptPyObject(v) => {
                let new_v: Vec<Option<u64>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::OptPyObject(new_v))
            },
            TinyColumn::Mixed(v) => {
                let new_v: Vec<ValueEnum> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| val.clone())
                    .collect();
                Ok(TinyColumn::Mixed(new_v))
            },
            TinyColumn::OptMixed(v) => {
                let new_v: Vec<Option<ValueEnum>> = v.iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| val.clone())
                    .collect();
                Ok(TinyColumn::OptMixed(new_v))
            },
        }
    }
}
