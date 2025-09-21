pub mod frame;
pub mod column;
pub mod simd;
pub mod parallel;
pub mod chunked;
pub mod benchmarks;
mod groupby;
mod join;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{TinyColumn, ValueEnum, TinyFrame};
    use pyo3::prelude::*;
    use pyo3::types::{PyDict, PyList};

    #[test]
    fn test_int_column_creation() {
        let col = TinyColumn::Int(vec![1, 2, 3]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_opt_int_column_creation() {
        let col = TinyColumn::OptInt(vec![Some(1), None, Some(3)]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_float_column_creation() {
        let col = TinyColumn::Float(vec![1.0, 2.5, 3.14]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_str_column_creation() {
        let col = TinyColumn::Str(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_bool_column_creation() {
        let col = TinyColumn::Bool(vec![true, false, true]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_mixed_column_creation() {
        let col = TinyColumn::Mixed(vec![
            ValueEnum::Int(42),
            ValueEnum::Str("hello".to_string()),
            ValueEnum::Float(3.14),
        ]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_opt_mixed_column_creation() {
        let col = TinyColumn::OptMixed(vec![
            Some(ValueEnum::Int(42)),
            None,
            Some(ValueEnum::Str("hello".to_string())),
        ]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_value_enum_variants() {
        let int_val = ValueEnum::Int(42);
        let float_val = ValueEnum::Float(3.14);
        let str_val = ValueEnum::Str("hello".to_string());
        let bool_val = ValueEnum::Bool(true);

        assert!(matches!(int_val, ValueEnum::Int(_)));
        assert!(matches!(float_val, ValueEnum::Float(_)));
        assert!(matches!(str_val, ValueEnum::Str(_)));
        assert!(matches!(bool_val, ValueEnum::Bool(_)));
    }

    #[test]
    fn test_empty_frame_creation() {
        let frame = TinyFrame::new();
        assert_eq!(frame.len(), 0);
        assert!(frame.is_empty());
        assert_eq!(frame.shape(), (0, 0));
    }

    #[test]
    fn test_frame_creation_from_dicts() {
        Python::with_gil(|py| {
            let mut records = vec![
                PyDict::new(py),
                PyDict::new(py),
            ];
            
            records[0].set_item("age", 25).unwrap();
            records[0].set_item("name", "Alice").unwrap();
            
            records[1].set_item("age", 30).unwrap();
            records[1].set_item("name", "Bob").unwrap();
            
            let records: Vec<PyObject> = records.into_iter().map(|dict| dict.into()).collect();
            let py_list = PyList::new(py, &records);
            let frame = TinyFrame::from_dicts(py, py_list.as_ref()).unwrap();
            
            assert_eq!(frame.len(), 2);
            assert!(!frame.is_empty());
            assert_eq!(frame.shape(), (2, 2));
        });
    }

    #[test]
    fn test_empty_records_error() {
        Python::with_gil(|py| {
            let empty_records: Vec<PyObject> = vec![];
            let py_list = PyList::new(py, &empty_records);
            let result = TinyFrame::from_dicts(py, py_list.as_ref());
            assert!(result.is_err());
        });
    }
}

use pyo3::prelude::*;
use crate::frame::TinyFrame;
use crate::groupby::TinyGroupBy;

#[pymodule]
fn feathertail(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TinyFrame>()?;
    m.add_class::<TinyGroupBy>()?;
    Ok(())
}
