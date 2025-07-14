use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::frame::{TinyColumn, TinyFrame, ValueEnum};

#[pyclass]
pub struct TinyFrameRowIter {
    #[pyo3(get)]
    frame: Py<TinyFrame>,
    index: usize,
}

#[pymethods]
impl TinyFrameRowIter {
    #[new]
    pub fn new(frame: Py<TinyFrame>) -> Self {
        TinyFrameRowIter { frame, index: 0 }
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>, py: Python) -> Option<PyObject> {
        {
            let frame_ref = slf.frame.borrow(py);

            if slf.index >= frame_ref.length {
                return None;
            }

            let row_dict = PyDict::new(py);

            for (col_name, col_data) in &frame_ref.columns {
                let val = match col_data {
                    TinyColumn::Int(v) => v[slf.index].into_py(py),
                    TinyColumn::Float(v) => v[slf.index].into_py(py),
                    TinyColumn::Str(v) => v[slf.index].clone().into_py(py),
                    TinyColumn::Bool(v) => v[slf.index].into_py(py),
                    TinyColumn::OptInt(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                    TinyColumn::OptFloat(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                    TinyColumn::OptStr(v) => v[slf.index].clone().map_or(py.None(), |x| x.into_py(py)),
                    TinyColumn::OptBool(v) => v[slf.index].map_or(py.None(), |x| x.into_py(py)),
                    TinyColumn::Mixed(v) => v[slf.index].to_py(py, &frame_ref.py_objects),
                    TinyColumn::OptMixed(v) => v[slf.index]
                        .as_ref()
                        .map_or(py.None(), |x| x.to_py(py, &frame_ref.py_objects)),
                    TinyColumn::PyObject(v) => {
                        let obj = frame_ref.py_objects.get(&v[slf.index]);
                        obj.map_or(py.None(), |o| o.clone_ref(py))
                    }
                    TinyColumn::OptPyObject(v) => {
                        v[slf.index]
                            .and_then(|id| frame_ref.py_objects.get(&id).map(|o| o.clone_ref(py)))
                            .unwrap_or_else(|| py.None())
                    }
                };

                row_dict.set_item(col_name, val).unwrap();
            }

            // Move row_dict out of the inner block so we can drop frame_ref first
            let result = row_dict.into();
            // Drop frame_ref here
            drop(frame_ref);

            slf.index += 1;
            Some(result)
        }
    }
}
