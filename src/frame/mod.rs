use pyo3::prelude::*;
use std::collections::HashMap;
use crate::frame::iter::TinyFrameRowIter;
use crate::column::TinyCol;


pub mod cast;
pub mod convert;
pub mod edit;
pub mod fillna;
pub mod iter;

#[derive(Clone)]
pub enum ValueEnum {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    PyObjectId(u64),
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
    fn new() -> Self {
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
    fn from_dicts(py: Python, records: &PyAny) -> PyResult<Self> {
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

    /// Return the number of rows.
    ///
    /// Returns:
    ///     int: Number of rows in the frame.
    fn len(&self) -> usize {
        self.length
    }

    /// Check if the frame is empty.
    ///
    /// Returns:
    ///     bool: True if empty, False otherwise.
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Shape of the frame (rows, columns).
    ///
    /// Returns:
    ///     Tuple[int, int]: (number of rows, number of columns).
    #[getter]
    fn shape(&self) -> (usize, usize) {
        (self.length, self.columns.len())
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
