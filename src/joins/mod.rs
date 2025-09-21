use std::collections::HashMap;
use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};

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
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("Left column '{}' not found", col)
                ));
            }
        }
        for col in &right_on {
            if !right.columns.contains_key(col) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Right column '{}' not found", col)
                ));
            }
        }

        if left_on.len() != right_on.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Number of join columns must match"
            ));
        }

        // Build hash maps for efficient lookups
        let left_keys = Self::build_key_map(left, &left_on)?;
        let right_keys = Self::build_key_map(right, &right_on)?;

        // Perform the join based on type
        match join_type {
            JoinType::Inner => Self::inner_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on),
            JoinType::Left => Self::left_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on),
            JoinType::Right => Self::right_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on),
            JoinType::Outer => Self::outer_join_impl(left, right, &left_keys, &right_keys, &left_on, &right_on),
        }
    }

    // Build a hash map of keys to row indices
    fn build_key_map(frame: &TinyFrame, columns: &[String]) -> PyResult<HashMap<Vec<ValueEnum>, Vec<usize>>> {
        let mut key_map: HashMap<Vec<ValueEnum>, Vec<usize>> = HashMap::new();

        for row_idx in 0..frame.length {
            let mut key = Vec::new();
            for col_name in columns {
                let col = frame.columns.get(col_name).unwrap();
                if let Some(value) = Self::get_value_at_index(col, row_idx) {
                    key.push(value);
                } else {
                    // Skip rows with null values in join columns
                    continue;
                }
            }
            key_map.entry(key).or_insert_with(Vec::new).push(row_idx);
        }

        Ok(key_map)
    }

    // Get value at specific index from a column
    fn get_value_at_index(col: &TinyColumn, index: usize) -> Option<ValueEnum> {
        match col {
            TinyColumn::Int(v) => v.get(index).map(|&val| ValueEnum::Int(val)),
            TinyColumn::Float(v) => v.get(index).map(|&val| ValueEnum::Float(val)),
            TinyColumn::Str(v) => v.get(index).map(|val| ValueEnum::Str(val.clone())),
            TinyColumn::Bool(v) => v.get(index).map(|&val| ValueEnum::Bool(val)),
            TinyColumn::PyObject(v) => v.get(index).map(|&val| ValueEnum::PyObjectId(val)),
            TinyColumn::Mixed(v) => v.get(index).and_then(|val| Some(val.clone())),
            TinyColumn::OptInt(v) => v.get(index).and_then(|val| val.map(ValueEnum::Int)),
            TinyColumn::OptFloat(v) => v.get(index).and_then(|val| val.map(ValueEnum::Float)),
            TinyColumn::OptStr(v) => v.get(index).and_then(|val| val.as_ref().map(|s| ValueEnum::Str(s.clone()))),
            TinyColumn::OptBool(v) => v.get(index).and_then(|val| val.map(ValueEnum::Bool)),
            TinyColumn::OptPyObject(v) => v.get(index).and_then(|val| val.map(ValueEnum::PyObjectId)),
            TinyColumn::OptMixed(v) => v.get(index).and_then(|val| val.clone()),
        }
    }

    // Inner join implementation
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
                let mut new_col = Self::create_empty_column(col_data)?;
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add right columns (excluding join columns)
        for (col_name, col_data) in &right.columns {
            if !right_on.contains(col_name) {
                let mut new_col = Self::create_empty_column(col_data)?;
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add join columns (from left)
        for col_name in left_on {
            let col_data = left.columns.get(col_name).unwrap();
            let mut new_col = Self::create_empty_column(col_data)?;
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
                                Self::append_value_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                    left_idx,
                                )?;
                            }
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            if !right_on.contains(col_name) {
                                Self::append_value_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                    right_idx,
                                )?;
                            }
                        }

                        // Add join key values
                        for (i, col_name) in left_on.iter().enumerate() {
                            let left_col = left.columns.get(col_name).unwrap();
                            Self::append_value_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                left_col,
                                left_idx,
                            )?;
                        }

                        result_length += 1;
                    }
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: left.py_objects.clone(),
        })
    }

    // Left join implementation
    fn left_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        left_on: &[String],
        right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add left columns
        for (col_name, col_data) in &left.columns {
            let mut new_col = Self::create_empty_column(col_data)?;
            result_columns.insert(col_name.clone(), new_col);
        }

        // Add right columns (excluding join columns) as optional
        for (col_name, col_data) in &right.columns {
            if !right_on.contains(col_name) {
                let new_col = Self::create_optional_column(col_data)?;
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
                            Self::append_value_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                                left_idx,
                            )?;
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            if !right_on.contains(col_name) {
                                Self::append_value_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                    right_idx,
                                )?;
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
                        Self::append_value_to_column(
                            result_columns.get_mut(col_name).unwrap(),
                            col_data,
                            left_idx,
                        )?;
                    }

                    // Add null values for right columns
                    for (col_name, col_data) in &right.columns {
                        if !right_on.contains(col_name) {
                            Self::append_null_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                            )?;
                        }
                    }

                    result_length += 1;
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: left.py_objects.clone(),
        })
    }

    // Right join implementation
    fn right_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        left_on: &[String],
        right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add left columns (excluding join columns) as optional
        for (col_name, col_data) in &left.columns {
            if !left_on.contains(col_name) {
                let new_col = Self::create_optional_column(col_data)?;
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Add right columns
        for (col_name, col_data) in &right.columns {
            let mut new_col = Self::create_empty_column(col_data)?;
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
                                Self::append_value_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                    left_idx,
                                )?;
                            }
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            Self::append_value_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                                right_idx,
                            )?;
                        }

                        result_length += 1;
                    }
                }
            } else {
                // Non-matching right rows (with nulls for left columns)
                for &right_idx in right_indices {
                    // Add null values for left columns
                    for (col_name, col_data) in &left.columns {
                        if !left_on.contains(col_name) {
                            Self::append_null_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                            )?;
                        }
                    }

                    // Add right row data
                    for (col_name, col_data) in &right.columns {
                        Self::append_value_to_column(
                            result_columns.get_mut(col_name).unwrap(),
                            col_data,
                            right_idx,
                        )?;
                    }

                    result_length += 1;
                }
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: right.py_objects.clone(),
        })
    }

    // Outer join implementation
    fn outer_join_impl(
        left: &TinyFrame,
        right: &TinyFrame,
        left_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        right_keys: &HashMap<Vec<ValueEnum>, Vec<usize>>,
        left_on: &[String],
        right_on: &[String],
    ) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add all columns from both frames as optional
        for (col_name, col_data) in &left.columns {
            let new_col = Self::create_optional_column(col_data)?;
            result_columns.insert(col_name.clone(), new_col);
        }

        for (col_name, col_data) in &right.columns {
            if !result_columns.contains_key(col_name) {
                let new_col = Self::create_optional_column(col_data)?;
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Collect all unique keys
        let mut all_keys = left_keys.keys().cloned().collect::<std::collections::HashSet<_>>();
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
                                Self::append_value_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                    left_idx,
                                )?;
                            }

                            // Add right row data
                            for (col_name, col_data) in &right.columns {
                                if !left.columns.contains_key(col_name) {
                                    Self::append_value_to_column(
                                        result_columns.get_mut(col_name).unwrap(),
                                        col_data,
                                        right_idx,
                                    )?;
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
                            Self::append_value_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                                left_idx,
                            )?;
                        }

                        // Add null values for right columns
                        for (col_name, col_data) in &right.columns {
                            if !left.columns.contains_key(col_name) {
                                Self::append_null_to_column(
                                    result_columns.get_mut(col_name).unwrap(),
                                    col_data,
                                )?;
                            }
                        }

                        result_length += 1;
                    }
                }
                (None, Some(right_idxs)) => {
                    // Only right side has this key
                    for &right_idx in right_idxs {
                        // Add null values for left columns
                        for (col_name, col_data) in &left.columns {
                            Self::append_null_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                            )?;
                        }

                        // Add right row data
                        for (col_name, col_data) in &right.columns {
                            Self::append_value_to_column(
                                result_columns.get_mut(col_name).unwrap(),
                                col_data,
                                right_idx,
                            )?;
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
            py_objects: left.py_objects.clone(),
        })
    }

    // Helper methods
    fn create_empty_column(col: &TinyColumn) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(_) => Ok(TinyColumn::Int(Vec::new())),
            TinyColumn::Float(_) => Ok(TinyColumn::Float(Vec::new())),
            TinyColumn::Str(_) => Ok(TinyColumn::Str(Vec::new())),
            TinyColumn::Bool(_) => Ok(TinyColumn::Bool(Vec::new())),
            TinyColumn::PyObject(_) => Ok(TinyColumn::PyObject(Vec::new())),
            TinyColumn::Mixed(_) => Ok(TinyColumn::Mixed(Vec::new())),
            TinyColumn::OptInt(_) => Ok(TinyColumn::OptInt(Vec::new())),
            TinyColumn::OptFloat(_) => Ok(TinyColumn::OptFloat(Vec::new())),
            TinyColumn::OptStr(_) => Ok(TinyColumn::OptStr(Vec::new())),
            TinyColumn::OptBool(_) => Ok(TinyColumn::OptBool(Vec::new())),
            TinyColumn::OptPyObject(_) => Ok(TinyColumn::OptPyObject(Vec::new())),
            TinyColumn::OptMixed(_) => Ok(TinyColumn::OptMixed(Vec::new())),
        }
    }

    fn create_optional_column(col: &TinyColumn) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(_) => Ok(TinyColumn::OptInt(Vec::new())),
            TinyColumn::Float(_) => Ok(TinyColumn::OptFloat(Vec::new())),
            TinyColumn::Str(_) => Ok(TinyColumn::OptStr(Vec::new())),
            TinyColumn::Bool(_) => Ok(TinyColumn::OptBool(Vec::new())),
            TinyColumn::PyObject(_) => Ok(TinyColumn::OptPyObject(Vec::new())),
            TinyColumn::Mixed(_) => Ok(TinyColumn::OptMixed(Vec::new())),
            TinyColumn::OptInt(_) => Ok(TinyColumn::OptInt(Vec::new())),
            TinyColumn::OptFloat(_) => Ok(TinyColumn::OptFloat(Vec::new())),
            TinyColumn::OptStr(_) => Ok(TinyColumn::OptStr(Vec::new())),
            TinyColumn::OptBool(_) => Ok(TinyColumn::OptBool(Vec::new())),
            TinyColumn::OptPyObject(_) => Ok(TinyColumn::OptPyObject(Vec::new())),
            TinyColumn::OptMixed(_) => Ok(TinyColumn::OptMixed(Vec::new())),
        }
    }

    fn append_value_to_column(col: &mut TinyColumn, source_col: &TinyColumn, source_idx: usize) -> PyResult<()> {
        match (col, source_col) {
            (TinyColumn::Int(target), TinyColumn::Int(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Float(target), TinyColumn::Float(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Str(target), TinyColumn::Str(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::Bool(target), TinyColumn::Bool(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::PyObject(target), TinyColumn::PyObject(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::Mixed(target), TinyColumn::Mixed(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::OptInt(target), TinyColumn::OptInt(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptFloat(target), TinyColumn::OptFloat(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptStr(target), TinyColumn::OptStr(source)) => {
                target.push(source[source_idx].clone());
            }
            (TinyColumn::OptBool(target), TinyColumn::OptBool(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptPyObject(target), TinyColumn::OptPyObject(source)) => {
                target.push(source[source_idx]);
            }
            (TinyColumn::OptMixed(target), TinyColumn::OptMixed(source)) => {
                target.push(source[source_idx].clone());
            }
            // Handle appending from non-optional to optional columns
            (TinyColumn::OptInt(target), TinyColumn::Int(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptFloat(target), TinyColumn::Float(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptStr(target), TinyColumn::Str(source)) => {
                target.push(Some(source[source_idx].clone()));
            }
            (TinyColumn::OptBool(target), TinyColumn::Bool(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptPyObject(target), TinyColumn::PyObject(source)) => {
                target.push(Some(source[source_idx]));
            }
            (TinyColumn::OptMixed(target), TinyColumn::Mixed(source)) => {
                target.push(Some(source[source_idx].clone()));
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Column type mismatch in join operation"
                ));
            }
        }
        Ok(())
    }

    fn append_null_to_column(col: &mut TinyColumn, source_col: &TinyColumn) -> PyResult<()> {
        match col {
            TinyColumn::OptInt(target) => {
                target.push(None);
            }
            TinyColumn::OptFloat(target) => {
                target.push(None);
            }
            TinyColumn::OptStr(target) => {
                target.push(None);
            }
            TinyColumn::OptBool(target) => {
                target.push(None);
            }
            TinyColumn::OptPyObject(target) => {
                target.push(None);
            }
            TinyColumn::OptMixed(target) => {
                target.push(None);
            }
            // Convert non-optional columns to optional when needed
            TinyColumn::Int(target) => {
                // Convert to OptInt
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptInt(opt_vec);
            }
            TinyColumn::Float(target) => {
                // Convert to OptFloat
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptFloat(opt_vec);
            }
            TinyColumn::Str(target) => {
                // Convert to OptStr
                let mut opt_vec = Vec::new();
                for val in target.iter() {
                    opt_vec.push(Some(val.clone()));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptStr(opt_vec);
            }
            TinyColumn::Bool(target) => {
                // Convert to OptBool
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptBool(opt_vec);
            }
            TinyColumn::PyObject(target) => {
                // Convert to OptPyObject
                let mut opt_vec = Vec::new();
                for &val in target.iter() {
                    opt_vec.push(Some(val));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptPyObject(opt_vec);
            }
            TinyColumn::Mixed(target) => {
                // Convert to OptMixed
                let mut opt_vec = Vec::new();
                for val in target.iter() {
                    opt_vec.push(Some(val.clone()));
                }
                opt_vec.push(None);
                *col = TinyColumn::OptMixed(opt_vec);
            }
            _ => {
                // For any other column types, just push None to the existing column
                // This handles cases where we can't convert to optional
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Cannot append null to this column type"
                ));
            }
        }
        Ok(())
    }
}

// Cross join implementation
impl JoinOps {
    pub fn cross_join(left: &TinyFrame, right: &TinyFrame) -> PyResult<TinyFrame> {
        let mut result_columns: HashMap<String, TinyColumn> = HashMap::new();
        let mut result_length = 0;

        // Add all columns from both frames
        for (col_name, col_data) in &left.columns {
            let mut new_col = Self::create_empty_column(col_data)?;
            result_columns.insert(col_name.clone(), new_col);
        }

        for (col_name, col_data) in &right.columns {
            if !result_columns.contains_key(col_name) {
                let mut new_col = Self::create_empty_column(col_data)?;
                result_columns.insert(col_name.clone(), new_col);
            }
        }

        // Perform cross join
        for left_idx in 0..left.length {
            for right_idx in 0..right.length {
                // Add left row data
                for (col_name, col_data) in &left.columns {
                    Self::append_value_to_column(
                        result_columns.get_mut(col_name).unwrap(),
                        col_data,
                        left_idx,
                    )?;
                }

                // Add right row data
                for (col_name, col_data) in &right.columns {
                    Self::append_value_to_column(
                        result_columns.get_mut(col_name).unwrap(),
                        col_data,
                        right_idx,
                    )?;
                }

                result_length += 1;
            }
        }

        Ok(TinyFrame {
            columns: result_columns,
            length: result_length,
            py_objects: left.py_objects.clone(),
        })
    }
}
