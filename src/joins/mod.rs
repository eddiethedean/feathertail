use crate::frame::{TinyColumn, TinyFrame, ValueEnum};
use pyo3::prelude::*;
use std::collections::HashMap;

mod key_index;
mod validation;

// Join types
#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
}

// Join operations for TinyFrame
pub struct JoinOps;

impl JoinOps {
    // Inner join
    pub fn inner_join(
        left: &TinyFrame,
        right: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<TinyFrame> {
        Self::perform_join(left, right, left_on, right_on, JoinType::Inner)
    }

    // Left join
    pub fn left_join(
        left: &TinyFrame,
        right: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<TinyFrame> {
        Self::perform_join(left, right, left_on, right_on, JoinType::Left)
    }

    // Right join
    pub fn right_join(
        left: &TinyFrame,
        right: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<TinyFrame> {
        Self::perform_join(left, right, left_on, right_on, JoinType::Right)
    }

    // Outer join
    pub fn outer_join(
        left: &TinyFrame,
        right: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<TinyFrame> {
        Self::perform_join(left, right, left_on, right_on, JoinType::Outer)
    }

    // Main join implementation
    fn perform_join(
        left: &TinyFrame,
        right: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
        join_type: JoinType,
    ) -> PyResult<TinyFrame> {
        // Validate join columns
        for col in &left_on {
            if !left.columns.contains_key(col) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Left column '{}' not found",
                    col
                )));
            }
        }
        for col in &right_on {
            if !right.columns.contains_key(col) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Right column '{}' not found",
                    col
                )));
            }
        }

        if left_on.len() != right_on.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of join columns must match",
            ));
        }

        validation::keyed_join_output_columns(left, right, &left_on, &right_on)?;

        // Build hash maps for efficient lookups
        let left_keys = key_index::build_key_map(left, &left_on)?;
        let right_keys = key_index::build_key_map(right, &right_on)?;

        // Perform the join based on type
        match join_type {
            JoinType::Inner => {
                Self::inner_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on)
            }
            JoinType::Left => {
                Self::left_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on)
            }
            JoinType::Right => {
                Self::right_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on)
            }
            JoinType::Outer => {
                Self::outer_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on)
            }
        }
    }

    /// Merge Python object fallback maps for join results. Detects incompatible identity reuse.
    fn merge_py_object_maps(
        left_m: &HashMap<u64, PyObject>,
        right_m: &HashMap<u64, PyObject>,
    ) -> PyResult<HashMap<u64, PyObject>> {
        let mut out = left_m.clone();
        for (&id, ro) in right_m {
            if let Some(lo) = out.get(&id) {
                if lo.as_ptr() != ro.as_ptr() {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "join: PyObjectId {} maps to differing objects on left and right; cannot merge safely",
                        id
                    )));
                }
            } else {
                out.insert(id, ro.clone());
            }
        }
        Ok(out)
    }

    fn inner_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        left_on: &[String],
        right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add left columns (excluding join columns)
        for (col_name, col_data) in &left.columns {
            if !left_on.contains(col_name) {
                let new_col = col_data.empty_same_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add right columns (excluding join columns)
        for (col_name, col_data) in &right.columns {
            if !right_on.contains(col_name) {
                let new_col = col_data.empty_same_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add join columns (from left)
        for col_name in left_on {
            let col_data = left.columns.get(col_name).unwrap();
            let new_col = col_data.empty_same_layout();
            result_columns.insert(col_name.clone(), new_col);
        }

        // Perform the join
        for (key, left_indices) in left_keys {
            if let Some(right_indices) = right_keys.get(key) {
                for &left_idx in left_indices {
                    for &right_idx in right_indices {
                        // Add left row data
                        for (col_name, col_data) in &left.columns {
                            if !left_on.contains(col_name) {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .append_row_join(col_data, left_idx)?;
                            }
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            if !right_on.contains(col_name) {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .append_row_join(col_data, right_idx)?;
                            }
                        }

                        // Add join key values
                        for (_i, col_name) in left_on.iter().enumerate() {
                            let left_col = left.columns.get(col_name).unwrap();
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .append_row_join(left_col, left_idx)?;
                        }

                        result_length += 1;
                    }
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: Self::merge_py_object_maps(&left.py_objects, &right.py_objects)?,
        })
    }

    // Left join implementation
    fn left_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        _left_on: &[String],
        right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add left columns
        for (col_name, col_data) in &left.columns {
            let new_col = col_data.empty_same_layout();
            result_columns.insert(col_name.clone(), new_col);
        }

        // Add right columns (excluding join columns) as optional
        for (col_name, col_data) in &right.columns {
            if !right_on.contains(col_name) {
                let new_col = col_data.empty_optional_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Perform the join
        for (key, left_indices) in left_keys {
            if let Some(right_indices) = right_keys.get(key) {
                // Matching rows
                for &left_idx in left_indices {
                    for &right_idx in right_indices {
                        // Add left row data
                        for (col_name, col_data) in &left.columns {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .append_row_join(col_data, left_idx)?;
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            if !right_on.contains(col_name) {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .append_row_join(col_data, right_idx)?;
                            }
                        }

                        result_length += 1;
                    }
                }
            } else {
                // Non-matching left rows (with nulls for right columns)
                for &left_idx in left_indices {
                    // Add left row data
                    for (col_name, col_data) in &left.columns {
                        result_columns
                            .get_mut(col_name)
                            .unwrap()
                            .append_row_join(col_data, left_idx)?;
                    }

                    // Add null values for right columns
                    for (col_name, _col_data) in &right.columns {
                        if !right_on.contains(col_name) {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .push_null_or_promote()?;
                        }
                    }

                    result_length += 1;
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: Self::merge_py_object_maps(&left.py_objects, &right.py_objects)?,
        })
    }

    // Right join implementation
    fn right_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        left_on: &[String],
        _right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add left columns (excluding join columns) as optional
        for (col_name, col_data) in &left.columns {
            if !left_on.contains(col_name) {
                let new_col = col_data.empty_optional_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add right columns
        for (col_name, col_data) in &right.columns {
            let new_col = col_data.empty_same_layout();
            result_columns.insert(col_name.clone(), new_col);
        }

        // Perform the join
        for (key, right_indices) in right_keys {
            if let Some(left_indices) = left_keys.get(key) {
                // Matching rows
                for &left_idx in left_indices {
                    for &right_idx in right_indices {
                        // Add left row data
                        for (col_name, col_data) in &left.columns {
                            if !left_on.contains(col_name) {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .append_row_join(col_data, left_idx)?;
                            }
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .append_row_join(col_data, right_idx)?;
                        }

                        result_length += 1;
                    }
                }
            } else {
                // Non-matching right rows (with nulls for left columns)
                for &right_idx in right_indices {
                    // Add null values for left columns
                    for (col_name, _col_data) in &left.columns {
                        if !left_on.contains(col_name) {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .push_null_or_promote()?;
                        }
                    }

                    // Add right row data
                    for (col_name, col_data) in &right.columns {
                        result_columns
                            .get_mut(col_name)
                            .unwrap()
                            .append_row_join(col_data, right_idx)?;
                    }

                    result_length += 1;
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: Self::merge_py_object_maps(&left.py_objects, &right.py_objects)?,
        })
    }

    // Outer join implementation
    fn outer_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        _left_on: &[String],
        _right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add all columns from both frames as optional
        for (col_name, col_data) in &left.columns {
            let new_col = col_data.empty_optional_layout();
            result_columns.insert(col_name.clone(), new_col);
        }

        for (col_name, col_data) in &right.columns {
            if !result_columns.contains_key(col_name) {
                let new_col = col_data.empty_optional_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Collect all unique keys
        let mut all_keys = left_keys
            .keys()
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        all_keys.extend(right_keys.keys().cloned());

        // Perform the join
        for key in all_keys {
            let left_indices = left_keys.get(&key);
            let right_indices = right_keys.get(&key);

            match (left_indices, right_indices) {
                (Some(left_idxs), Some(right_idxs)) => {
                    // Both sides have matching keys
                    for &left_idx in left_idxs {
                        for &right_idx in right_idxs {
                            // Add left row data
                            for (col_name, col_data) in &left.columns {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .append_row_join(col_data, left_idx)?;
                            }

                            // Add right row data
                            for (col_name, col_data) in &right.columns {
                                if !left.columns.contains_key(col_name) {
                                    result_columns
                                        .get_mut(col_name)
                                        .unwrap()
                                        .append_row_join(col_data, right_idx)?;
                                }
                            }

                            result_length += 1;
                        }
                    }
                }
                (Some(left_idxs), None) => {
                    // Only left side has this key
                    for &left_idx in left_idxs {
                        // Add left row data
                        for (col_name, col_data) in &left.columns {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .append_row_join(col_data, left_idx)?;
                        }

                        // Add null values for right columns
                        for (col_name, _col_data) in &right.columns {
                            if !left.columns.contains_key(col_name) {
                                result_columns
                                    .get_mut(col_name)
                                    .unwrap()
                                    .push_null_or_promote()?;
                            }
                        }

                        result_length += 1;
                    }
                }
                (None, Some(right_idxs)) => {
                    // Only right side has this key
                    for &right_idx in right_idxs {
                        // Add null values for left columns
                        for (col_name, _col_data) in &left.columns {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .push_null_or_promote()?;
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            result_columns
                                .get_mut(col_name)
                                .unwrap()
                                .append_row_join(col_data, right_idx)?;
                        }

                        result_length += 1;
                    }
                }
                (None, None) => unreachable!(),
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: Self::merge_py_object_maps(&left.py_objects, &right.py_objects)?,
        })
    }
}

// Cross join implementation
impl JoinOps {
    pub fn cross_join(left: &TinyFrame, right: &TinyFrame) -> PyResult<TinyFrame> {
        validation::cross_join_columns_disjoint(left, right)?;

        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add all columns from both frames
        for (col_name, col_data) in &left.columns {
            let new_col = col_data.empty_same_layout();
            result_columns.insert(col_name.clone(), new_col);
        }

        for (col_name, col_data) in &right.columns {
            if !result_columns.contains_key(col_name) {
                let new_col = col_data.empty_same_layout();
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Perform cross join
        for left_idx in 0..left.length {
            for right_idx in 0..right.length {
                // Add left row data
                for (col_name, col_data) in &left.columns {
                    result_columns
                        .get_mut(col_name)
                        .unwrap()
                        .append_row_join(col_data, left_idx)?;
                }

                // Add right row data
                for (col_name, col_data) in &right.columns {
                    result_columns
                        .get_mut(col_name)
                        .unwrap()
                        .append_row_join(col_data, right_idx)?;
                }

                result_length += 1;
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: Self::merge_py_object_maps(&left.py_objects, &right.py_objects)?,
        })
    }
}
