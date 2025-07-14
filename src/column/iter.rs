use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn as FrameColumn, ValueEnum};
use super::TinyCol;

#[pyclass]
pub struct TinyColIter {
    frame: Py<TinyFrame>,
    col_name: String,
    index: usize,
}

#[pymethods]
impl TinyColIter {
    #[new]
    pub fn new(col: PyRef<TinyCol>, py: Python) -> PyResult<Py<Self>> {
        let iter = TinyColIter {
            frame: col.frame.clone(),
            col_name: col.name.clone(),
            index: 0,
        };
        Py::new(py, iter)
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>, py: Python) -> Option<PyObject> {
        let val = {
            let frame = slf.frame.borrow(py);
            let col = frame.columns.get(&slf.col_name)?;

            if slf.index >= frame.length {
                return None;
            }

            match col {
                FrameColumn::Int(v) => v[slf.index].into_py(py),
                FrameColumn::Float(v) => v[slf.index].into_py(py),
                FrameColumn::Str(v) => v[slf.index].clone().into_py(py),
                FrameColumn::Bool(v) => v[slf.index].into_py(py),
                FrameColumn::OptInt(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                FrameColumn::OptFloat(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                FrameColumn::OptStr(v) => v[slf.index].clone().map_or(py.None(), |x| x.into_py(py)),
                FrameColumn::OptBool(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                FrameColumn::Mixed(v) => v[slf.index].to_py(py, &frame.py_objects),
                FrameColumn::OptMixed(v) => v[slf.index]
                    .as_ref()
                    .map_or(py.None(), |x| x.to_py(py, &frame.py_objects)),
                FrameColumn::PyObject(v) => {
                    let id = v[slf.index];
                    frame.py_objects.get(&id).cloned().unwrap_or_else(|| py.None())
                }
                FrameColumn::OptPyObject(v) => v[slf.index]
                    .and_then(|id| frame.py_objects.get(&id).cloned())
                    .unwrap_or_else(|| py.None()),
            }
        };

        slf.index += 1;
        Some(val)
    }
}
