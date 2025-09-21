pub mod frame;
pub mod column;
pub mod simd;
pub mod parallel;
pub mod chunked;
pub mod benchmarks;
pub mod joins;
pub mod stats;
pub mod types;
pub mod timeseries;
pub mod window;
pub mod ranking;
pub mod string; // Added for string operations
pub mod validation; // Added for data validation
pub mod logging; // Added for logging system
pub mod debug; // Added for debug tools
pub mod profiling; // Added for performance profiling
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
use std::collections::HashMap;

// Logging function wrappers
#[pyfunction]
fn init_logging() -> PyResult<()> {
    crate::logging::init_logging()
}

#[pyfunction]
fn init_logging_with_config(level: &str, log_memory: bool, log_performance: bool, log_operations: bool) -> PyResult<()> {
    crate::logging::init_logging_with_config(level, log_memory, log_performance, log_operations)
}

#[pyfunction]
fn log_operation(operation: &str, details: &str) {
    crate::logging::log_operation(operation, details);
}

#[pyfunction]
fn log_memory_usage(operation: &str, memory_mb: f64) {
    crate::logging::log_memory_usage(operation, memory_mb);
}

#[pyfunction]
fn log_performance(operation: &str, duration_ms: f64, rows_processed: usize) {
    crate::logging::log_performance(operation, duration_ms, rows_processed);
}

#[pyfunction]
fn log_error(operation: &str, error: &str, context: Option<&str>) {
    crate::logging::log_error(operation, error, context);
}

#[pyfunction]
fn log_warning(operation: &str, warning: &str, context: Option<&str>) {
    crate::logging::log_warning(operation, warning, context);
}

// Debug function wrappers
#[pyfunction]
fn enable_debug() {
    crate::debug::enable_debug();
}

#[pyfunction]
fn disable_debug() {
    crate::debug::disable_debug();
}

#[pyfunction]
fn is_debug_enabled() -> bool {
    crate::debug::is_debug_enabled()
}

#[pyfunction]
fn log_debug_info(operation: &str, duration_ms: f64, memory_mb: f64, rows_processed: usize) {
    let mut info = crate::debug::create_debug_info(operation);
    info.set_memory_after(memory_mb);
    info.set_rows_processed(rows_processed);
    crate::debug::log_debug_info(&info);
}

#[pyfunction]
fn log_operation_start(operation: &str) {
    crate::debug::log_operation_start(operation);
}

#[pyfunction]
fn log_operation_end(operation: &str, duration_ms: f64) {
    crate::debug::log_operation_end(operation, duration_ms);
}

// Profiling function wrappers
#[pyfunction]
fn enable_profiling() {
    crate::profiling::enable_profiling();
}

#[pyfunction]
fn disable_profiling() {
    crate::profiling::disable_profiling();
}

#[pyfunction]
fn is_profiling_enabled() -> bool {
    crate::profiling::is_profiling_enabled()
}

#[pyfunction]
fn get_operation_stats(operation: &str) -> Option<HashMap<String, f64>> {
    crate::profiling::get_operation_stats(operation)
}

#[pyfunction]
fn get_overall_stats() -> HashMap<String, f64> {
    crate::profiling::get_overall_stats()
}

#[pyfunction]
fn clear_profiling_data() {
    crate::profiling::clear_profiling_data();
}

#[pyfunction]
fn print_profiling_report() {
    crate::profiling::print_profiling_report();
}

#[pymodule]
fn feathertail(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TinyFrame>()?;
    m.add_class::<TinyGroupBy>()?;
    
    // Add logging functions
    m.add_function(wrap_pyfunction!(init_logging, m)?)?;
    m.add_function(wrap_pyfunction!(init_logging_with_config, m)?)?;
    m.add_function(wrap_pyfunction!(log_operation, m)?)?;
    m.add_function(wrap_pyfunction!(log_memory_usage, m)?)?;
    m.add_function(wrap_pyfunction!(log_performance, m)?)?;
    m.add_function(wrap_pyfunction!(log_error, m)?)?;
    m.add_function(wrap_pyfunction!(log_warning, m)?)?;
    
    // Add debug functions
    m.add_function(wrap_pyfunction!(enable_debug, m)?)?;
    m.add_function(wrap_pyfunction!(disable_debug, m)?)?;
    m.add_function(wrap_pyfunction!(is_debug_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(log_debug_info, m)?)?;
    m.add_function(wrap_pyfunction!(log_operation_start, m)?)?;
    m.add_function(wrap_pyfunction!(log_operation_end, m)?)?;
    
    // Add profiling functions
    m.add_function(wrap_pyfunction!(enable_profiling, m)?)?;
    m.add_function(wrap_pyfunction!(disable_profiling, m)?)?;
    m.add_function(wrap_pyfunction!(is_profiling_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(get_operation_stats, m)?)?;
    m.add_function(wrap_pyfunction!(get_overall_stats, m)?)?;
    m.add_function(wrap_pyfunction!(clear_profiling_data, m)?)?;
    m.add_function(wrap_pyfunction!(print_profiling_report, m)?)?;
    
    Ok(())
}
