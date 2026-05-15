use pyo3::prelude::*;

mod iter;

use crate::frame::TinyFrame;

#[pyclass]
pub struct TinyCol {
    pub name: String,
    pub frame: Py<TinyFrame>,
}

#[pymethods]
impl TinyCol {
    #[getter]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    pub fn type_str(&self, py: Python) -> PyResult<String> {
        let frame = self.frame.borrow(py);
        let col = frame.columns.get(&self.name)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", self.name)))?;
        let type_str = col.type_tag();
        Ok(type_str.into())
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let type_str = self.type_str(py)?;
        Ok(format!("TinyCol(name='{}', type='{}')", self.name, type_str))
    }

    fn __iter__(slf: PyRef<Self>, py: Python) -> PyResult<Py<iter::TinyColIter>> {
        iter::TinyColIter::new(slf, py)
    }
}
