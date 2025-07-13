pub mod frame;
mod groupby;
mod join;
mod utils;

use pyo3::prelude::*;
use crate::frame::TinyFrame;
use crate::groupby::TinyGroupBy;

#[pymodule]
fn feathertail(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TinyFrame>()?;
    m.add_class::<TinyGroupBy>()?;
    Ok(())
}
