use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn};
use std::collections::HashMap;

/// String operations for TinyFrame
pub struct StringOps;

impl StringOps {
    /// Helper to extract string values from a column
    fn extract_string_values(frame: &TinyFrame, column: &str) -> PyResult<Vec<Option<String>>> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        match col {
            TinyColumn::Str(v) => Ok(v.iter().map(|s| Some(s.clone())).collect()),
            TinyColumn::OptStr(v) => Ok(v.clone()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "String operations only supported on string columns"
            )),
        }
    }

    /// Convert strings to uppercase
    pub fn str_upper_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<String>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.to_uppercase()))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_upper", column), TinyColumn::OptStr(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Convert strings to lowercase
    pub fn str_lower_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<String>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.to_lowercase()))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_lower", column), TinyColumn::OptStr(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Strip whitespace from strings
    pub fn str_strip_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<String>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.trim().to_string()))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_strip", column), TinyColumn::OptStr(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Replace substrings in strings
    pub fn str_replace_impl(frame: &TinyFrame, column: &str, from: &str, to: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<String>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.replace(from, to)))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_replace", column), TinyColumn::OptStr(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Split strings by delimiter
    pub fn str_split_impl(frame: &TinyFrame, column: &str, delimiter: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<Vec<String>>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.split(delimiter).map(|part| part.to_string()).collect()))
            .collect();

        // Convert to a list of strings representation
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_split", column), TinyColumn::OptStr(
            result_values.iter()
                .map(|opt_vec| opt_vec.as_ref().map(|vec| vec.join("|")))
                .collect()
        ));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Check if strings contain substring
    pub fn str_contains_impl(frame: &TinyFrame, column: &str, substring: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<bool>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.contains(substring)))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_contains", column), TinyColumn::OptBool(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Get string length
    pub fn str_len_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let result_values: Vec<Option<i64>> = values.iter()
            .map(|opt_str| opt_str.as_ref().map(|s| s.len() as i64))
            .collect();

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_len", column), TinyColumn::OptInt(result_values));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Concatenate strings
    pub fn str_cat_impl(frame: &TinyFrame, column: &str, separator: &str) -> PyResult<TinyFrame> {
        let values = Self::extract_string_values(frame, column)?;
        let non_null_values: Vec<String> = values.iter()
            .filter_map(|opt_str| opt_str.as_ref().cloned())
            .collect();
        
        let concatenated = non_null_values.join(separator);
        
        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_cat", column), TinyColumn::Str(vec![concatenated; frame.length]));

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }
}
