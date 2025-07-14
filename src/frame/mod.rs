use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

pub mod cast;
pub mod convert;
pub mod edit;
pub mod fillna;

#[derive(Clone)]
pub enum ValueEnum {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    PyObjectId(u64), // fallback python object reference by id
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
}

#[pyclass]
pub struct TinyFrame {
    pub columns: HashMap<String, TinyColumn>,
    pub length: usize,
    pub py_objects: HashMap<u64, PyObject>, // fallback Python object storage
}

#[pymethods]
impl TinyFrame {
    #[new]
    fn new() -> Self {
        TinyFrame {
            columns: HashMap::new(),
            length: 0,
            py_objects: HashMap::new(),
        }
    }

    #[staticmethod]
    fn from_dicts(py: Python, records: &PyAny) -> PyResult<Self> {
        convert::from_dicts_impl(py, records)
    }

    fn to_dicts(&self, py: Python) -> PyResult<Vec<PyObject>> {
        convert::to_dicts_impl(self, py)
    }

    #[getter]
    fn shape(&self) -> (usize, usize) {
        (self.length, self.columns.len())
    }

    fn fillna(&mut self, py: Python, value: &PyAny) -> PyResult<()> {
        fillna::fillna_impl(self, py, value)
    }

    fn cast_column(&mut self, py: Python, column_name: String, new_type: &PyAny) -> PyResult<()> {
        cast::cast_column_impl(self, py, column_name, new_type)
    }

    fn edit_column(&mut self, py: Python, column_name: String, func: PyObject) -> PyResult<()> {
        edit::edit_column_impl(self, py, column_name, func)
    }

    fn drop_columns(&mut self, columns_to_drop: Vec<String>) -> PyResult<()> {
        for col_name in columns_to_drop {
            self.columns.remove(&col_name);
        }
        Ok(())
    }

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

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

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
}
