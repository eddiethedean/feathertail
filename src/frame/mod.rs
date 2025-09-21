use pyo3::prelude::*;
use std::collections::HashMap;
use crate::frame::iter::TinyFrameRowIter;
use crate::column::TinyCol;


pub mod cast;
pub mod convert;
pub mod edit;
pub mod fillna;
pub mod iter;
pub mod lazy;
pub mod optimize;
pub mod string_optimize;

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
                // Handle NaN and infinity for hashing
                if f.is_nan() {
                    state.write_u64(0x7ff8000000000000u64); // NaN representation
                } else if f.is_infinite() {
                    if f.is_sign_positive() {
                        state.write_u64(0x7ff0000000000000u64); // +inf
                    } else {
                        state.write_u64(0xfff0000000000000u64); // -inf
                    }
                } else {
                    f.to_bits().hash(state);
                }
            },
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
            (ValueEnum::Float(a), ValueEnum::Float(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
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

/// TinyFrame
///
/// A fast, flexible DataFrame-like structure implemented in Rust for Python.
///
/// Supports type inference, optional and mixed columns, fillna, casting, editing, and row-wise iteration.
#[pyclass]
#[derive(Clone)]
pub struct TinyFrame {
    pub columns: HashMap<String, TinyColumn>,
    pub length: usize,
    pub py_objects: HashMap<u64, PyObject>,
}

#[pymethods]
impl TinyFrame {
    /// Create a new empty TinyFrame.
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        TinyFrame {
            columns: HashMap::new(),
            length: 0,
            py_objects: HashMap::new(),
        }
    }

    /// Create a TinyFrame from a list of Python dictionaries.
    ///
    /// Args:
    ///     records (List[dict]): List of Python dictionaries.
    ///
    /// Returns:
    ///     TinyFrame: New frame inferred from the records.
    #[staticmethod]
    #[pyo3(text_signature = "(records)")]
    pub fn from_dicts(py: Python, records: &PyAny) -> PyResult<Self> {
        convert::from_dicts_impl(py, records)
    }

    /// Convert the TinyFrame to a list of dictionaries.
    ///
    /// Returns:
    ///     List[dict]: Frame data as a list of dicts.
    fn to_dicts(&self, py: Python) -> PyResult<Vec<PyObject>> {
        convert::to_dicts_impl(self, py)
    }

    /// Fill missing (None) values in the frame.
    ///
    /// Args:
    ///     value (dict or scalar): Dictionary mapping column names to fill values or a single scalar value.
    fn fillna(&mut self, py: Python, value: &PyAny) -> PyResult<()> {
        fillna::fillna_impl(self, py, value)
    }

    /// Cast a column to a different type.
    ///
    /// Args:
    ///     column_name (str): Name of the column.
    ///     new_type (type): Target Python type (e.g., int, float, str, bool).
    fn cast_column(&mut self, py: Python, column_name: String, new_type: &PyAny) -> PyResult<()> {
        cast::cast_column_impl(self, py, column_name, new_type)
    }

    /// Edit a column using a custom Python function.  
    ///
    /// Args:
    ///     column_name (str): Name of the column.
    ///     func (callable): Python function to apply to each value.
    fn edit_column(&mut self, py: Python, column_name: String, func: PyObject) -> PyResult<()> {
        edit::edit_column_impl(self, py, column_name, func)
    }

    /// Drop specified columns from the frame.
    ///
    /// Args:
    ///     columns_to_drop (List[str]): List of column names to remove.
    fn drop_columns(&mut self, columns_to_drop: Vec<String>) -> PyResult<()> {
        for col_name in columns_to_drop {
            if !self.columns.contains_key(&col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("Column '{}' not found", col_name)
                ));
            }
            self.columns.remove(&col_name);
        }
        Ok(())
    }

    /// Rename a column.
    ///
    /// Args:
    ///     old_name (str): Original column name.
    ///     new_name (str): New column name.
    fn rename_column(&mut self, old_name: String, new_name: String) -> PyResult<()> {
        if !self.columns.contains_key(&old_name) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", old_name)));
        }
        if self.columns.contains_key(&new_name) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{}' already exists", new_name)));
        }
        let col = self.columns.remove(&old_name).unwrap();
        self.columns.insert(new_name, col);
        Ok(())
    }

    /// Filter rows based on a condition.
    ///
    /// Args:
    ///     column (str): Column name to filter on.
    ///     condition (str): Condition operator ('==', '!=', '>', '<', '>=', '<=', 'in', 'not_in').
    ///     value: Value to compare against.
    ///
    /// Returns:
    ///     TinyFrame: New frame with filtered rows.
    fn filter(&self, py: Python, column: String, condition: String, value: &PyAny) -> PyResult<Self> {
        let col = self.columns.get(&column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column))
        })?;

        let mut filtered_indices = Vec::new();
        
        for (idx, val) in col.iter().enumerate() {
            let matches = match condition.as_str() {
                "==" => {
                    match val {
                        ValueEnum::Str(_) => self.compare_string_values(&val, value, py, "==")?,
                        _ => self.compare_values(&val, value, py, |a, b| a == b)?,
                    }
                }
                "!=" => {
                    match val {
                        ValueEnum::Str(_) => self.compare_string_values(&val, value, py, "!=")?,
                        _ => self.compare_values(&val, value, py, |a, b| a != b)?,
                    }
                }
                ">" => self.compare_values(&val, value, py, |a, b| a > b)?,
                "<" => self.compare_values(&val, value, py, |a, b| a < b)?,
                ">=" => self.compare_values(&val, value, py, |a, b| a >= b)?,
                "<=" => self.compare_values(&val, value, py, |a, b| a <= b)?,
                "in" => self.check_in_values(&val, value, py)?,
                "not_in" => !self.check_in_values(&val, value, py)?,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown condition: {}", condition)
                )),
            };
            
            if matches {
                filtered_indices.push(idx);
            }
        }

        self.filter_by_indices(filtered_indices)
    }

    /// Filter rows where column values are not null.
    ///
    /// Args:
    ///     column (str): Column name to check for nulls.
    ///
    /// Returns:
    ///     TinyFrame: New frame with non-null rows.
    fn dropna(&self, column: String) -> PyResult<Self> {
        let col = self.columns.get(&column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column))
        })?;

        let mut filtered_indices = Vec::new();
        
        for idx in 0..self.length {
            let is_not_null = match col {
                TinyColumn::OptInt(v) => v[idx].is_some(),
                TinyColumn::OptFloat(v) => v[idx].is_some(),
                TinyColumn::OptStr(v) => v[idx].is_some(),
                TinyColumn::OptBool(v) => v[idx].is_some(),
                TinyColumn::OptMixed(v) => v[idx].is_some(),
                TinyColumn::OptPyObject(v) => v[idx].is_some(),
                _ => true, // Non-optional columns are never null
            };
            
            if is_not_null {
                filtered_indices.push(idx);
            }
        }

        self.filter_by_indices(filtered_indices)
    }

    /// Sort the frame by one or more columns.
    ///
    /// Args:
    ///     by (List[str]): Column names to sort by.
    ///     ascending (bool): Sort in ascending order (default True).
    ///
    /// Returns:
    ///     TinyFrame: New frame with sorted rows.
    fn sort_values(&self, by: Vec<String>, ascending: Option<bool>) -> PyResult<Self> {
        let ascending = ascending.unwrap_or(true);
        
        // Handle empty frame
        if self.length == 0 {
            return Ok(self.clone());
        }
        
        // Validate all columns exist
        for col_name in &by {
            if !self.columns.contains_key(col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("Column '{}' not found", col_name)
                ));
            }
        }
        
        let mut indices: Vec<usize> = (0..self.length).collect();
        
        // Sort by multiple columns (stable sort)
        indices.sort_by(|&a, &b| {
            for col_name in &by {
                let col = match self.columns.get(col_name) {
                    Some(col) => col,
                    None => {
                        // This should not happen as we check columns exist before sorting
                        return std::cmp::Ordering::Equal;
                    }
                };
                
                let val_a = self.get_value_at_index(col, a);
                let val_b = self.get_value_at_index(col, b);
                
                let comparison = self.compare_for_sort(val_a, val_b);
                if comparison != std::cmp::Ordering::Equal {
                    return if ascending {
                        comparison
                    } else {
                        comparison.reverse()
                    };
                }
            }
            std::cmp::Ordering::Equal
        });

        self.filter_by_indices(indices)
    }

    /// Return the number of rows.
    ///
    /// Returns:
    ///     int: Number of rows in the frame.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the frame is empty.
    ///
    /// Returns:
    ///     bool: True if empty, False otherwise.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Shape of the frame (rows, columns).
    ///
    /// Returns:
    ///     Tuple[int, int]: (number of rows, number of columns).
    #[getter]
    pub fn shape(&self) -> (usize, usize) {
        (self.length, self.columns.len())
    }

    /// Get column names.
    ///
    /// Returns:
    ///     List[str]: List of column names
    #[getter]
    pub fn columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    /// Return string representation of the frame.
    fn __repr__(&self) -> String {
        let mut col_strs = Vec::new();
        for (name, col) in &self.columns {
            let type_str = match col {
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
            };
            col_strs.push(format!("'{}': '{}'", name, type_str));
        }
        format!(
            "TinyFrame(rows={}, columns={}, cols={{ {} }})",
            self.length,
            col_strs.len(),
            col_strs.join(", ")
        )
    }

    /// Iterate over rows as dictionaries.
    fn __iter__(slf: PyRef<Self>) -> PyResult<crate::frame::iter::TinyFrameRowIter> {
        Ok(crate::frame::iter::TinyFrameRowIter::new(slf.into()))
    }

    fn col(&self, py: Python, name: String) -> PyResult<Py<TinyCol>> {
        if !self.columns.contains_key(&name) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", name)));
        }

        let frame_clone = self.clone();
        let py_frame = Py::new(py, frame_clone)?;

        let col = TinyCol {
            name,
            frame: py_frame,
        };
        Py::new(py, col)
    }

    /// Inner join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the inner join
    pub fn inner_join(&self, other: &TinyFrame, left_on: Vec<String>, right_on: Vec<String>) -> PyResult<Self> {
        crate::joins::JoinOps::inner_join(self, other, left_on, right_on)
    }

    /// Left join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the left join
    pub fn left_join(&self, other: &TinyFrame, left_on: Vec<String>, right_on: Vec<String>) -> PyResult<Self> {
        crate::joins::JoinOps::left_join(self, other, left_on, right_on)
    }

    /// Right join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the right join
    pub fn right_join(&self, other: &TinyFrame, left_on: Vec<String>, right_on: Vec<String>) -> PyResult<Self> {
        crate::joins::JoinOps::right_join(self, other, left_on, right_on)
    }

    /// Outer join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the outer join
    pub fn outer_join(&self, other: &TinyFrame, left_on: Vec<String>, right_on: Vec<String>) -> PyResult<Self> {
        crate::joins::JoinOps::outer_join(self, other, left_on, right_on)
    }

    /// Cross join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///
    /// Returns:
    ///     TinyFrame: The result of the cross join
    pub fn cross_join(&self, other: &TinyFrame) -> PyResult<Self> {
        crate::joins::JoinOps::cross_join(self, other)
    }

}

impl TinyFrame {
    // Helper methods for filtering and sorting
    fn filter_by_indices(&self, indices: Vec<usize>) -> PyResult<Self> {
        let mut new_columns: HashMap<String, TinyColumn> = HashMap::new();
        
        for (name, column) in &self.columns {
            let new_column = match column {
                TinyColumn::Int(v) => {
                    let new_v: Vec<i64> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::Int(new_v)
                }
                TinyColumn::Float(v) => {
                    let new_v: Vec<f64> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::Float(new_v)
                }
                TinyColumn::Str(v) => {
                    let new_v: Vec<String> = indices.iter().map(|&i| v[i].clone()).collect();
                    TinyColumn::Str(new_v)
                }
                TinyColumn::Bool(v) => {
                    let new_v: Vec<bool> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::Bool(new_v)
                }
                TinyColumn::OptInt(v) => {
                    let new_v: Vec<Option<i64>> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::OptInt(new_v)
                }
                TinyColumn::OptFloat(v) => {
                    let new_v: Vec<Option<f64>> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::OptFloat(new_v)
                }
                TinyColumn::OptStr(v) => {
                    let new_v: Vec<Option<String>> = indices.iter().map(|&i| v[i].clone()).collect();
                    TinyColumn::OptStr(new_v)
                }
                TinyColumn::OptBool(v) => {
                    let new_v: Vec<Option<bool>> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::OptBool(new_v)
                }
                TinyColumn::Mixed(v) => {
                    let new_v: Vec<ValueEnum> = indices.iter().map(|&i| v[i].clone()).collect();
                    TinyColumn::Mixed(new_v)
                }
                TinyColumn::OptMixed(v) => {
                    let new_v: Vec<Option<ValueEnum>> = indices.iter().map(|&i| v[i].clone()).collect();
                    TinyColumn::OptMixed(new_v)
                }
                TinyColumn::PyObject(v) => {
                    let new_v: Vec<u64> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::PyObject(new_v)
                }
                TinyColumn::OptPyObject(v) => {
                    let new_v: Vec<Option<u64>> = indices.iter().map(|&i| v[i]).collect();
                    TinyColumn::OptPyObject(new_v)
                }
            };
            new_columns.insert(name.clone(), new_column);
        }

        Ok(TinyFrame {
            columns: new_columns,
            length: indices.len(),
            py_objects: self.py_objects.clone(),
        })
    }

    fn compare_values(&self, val: &ValueEnum, py_value: &PyAny, _py: Python, compare_fn: fn(f64, f64) -> bool) -> PyResult<bool> {
        match val {
            ValueEnum::Int(v) => {
                let py_f64 = py_value.extract::<f64>()?;
                Ok(compare_fn(*v as f64, py_f64))
            }
            ValueEnum::Float(v) => {
                let py_f64 = py_value.extract::<f64>()?;
                Ok(compare_fn(*v, py_f64))
            }
            ValueEnum::Str(_) => return Ok(false), // String comparison handled separately
            ValueEnum::Bool(_) => return Ok(false), // Bool comparison not implemented yet
            ValueEnum::PyObjectId(_) => return Ok(false), // PyObject comparison not implemented yet
        }
    }

    fn compare_string_values(&self, val: &ValueEnum, py_value: &PyAny, _py: Python, condition: &str) -> PyResult<bool> {
        match val {
            ValueEnum::Str(s) => {
                let py_str: String = py_value.extract()?;
                Ok(match condition {
                    "==" => s == &py_str,
                    "!=" => s != &py_str,
                    _ => false, // Other comparisons not supported for strings
                })
            }
            _ => Ok(false),
        }
    }

    fn check_in_values(&self, val: &ValueEnum, py_value: &PyAny, _py: Python) -> PyResult<bool> {
        // This is a simplified implementation - in practice, you'd want to handle different types
        match val {
            ValueEnum::Str(s) => {
                let py_str: String = py_value.extract()?;
                Ok(s == &py_str)
            }
            _ => Ok(false), // Other types not implemented yet
        }
    }

    fn get_value_at_index(&self, column: &TinyColumn, index: usize) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => Some(v[index] as f64),
            TinyColumn::Float(v) => Some(v[index]),
            TinyColumn::OptInt(v) => v[index].map(|x| x as f64),
            TinyColumn::OptFloat(v) => v[index],
            _ => None,
        }
    }

    fn compare_for_sort(&self, val_a: Option<f64>, val_b: Option<f64>) -> std::cmp::Ordering {
        match (val_a, val_b) {
            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }
}

impl TinyColumn {
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

    pub fn iter(&self) -> TinyColumnIter {
        TinyColumnIter::new(self)
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

impl ValueEnum {
    pub fn to_py(&self, py: Python, py_objects: &HashMap<u64, PyObject>) -> PyObject {
        match self {
            ValueEnum::Int(v) => v.into_py(py),
            ValueEnum::Float(v) => v.into_py(py),
            ValueEnum::Str(v) => v.clone().into_py(py),
            ValueEnum::Bool(v) => v.into_py(py),
            ValueEnum::PyObjectId(id) => {
                py_objects.get(id).map(|o| o.clone_ref(py)).unwrap_or_else(|| py.None())
            }
        }
    }
}
