use crate::frame::{TinyColumn, TinyFrame, ValueEnum};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

// Parallel processing operations for TinyFrame
pub struct ParallelOps;

/// Fixed-length group key: one optional value per grouping column (missing/null slots preserved).
pub(crate) type GroupKeyRow = Vec<Option<ValueEnum>>;

impl ParallelOps {
    pub(crate) fn build_group_key_row(
        frame: &TinyFrame,
        group_keys: &[String],
        row_idx: usize,
    ) -> GroupKeyRow {
        group_keys
            .iter()
            .map(|col_name| {
                frame
                    .columns
                    .get(col_name)
                    .and_then(|col| Self::get_value_at_index(col, row_idx))
            })
            .collect()
    }

    pub(crate) fn validate_group_key_columns(
        frame: &TinyFrame,
        group_keys: &[String],
    ) -> PyResult<()> {
        for col_name in group_keys {
            if !frame.columns.contains_key(col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Column '{}' not found",
                    col_name
                )));
            }
        }
        Ok(())
    }

    // Parallel GroupBy operations
    pub fn parallel_groupby_sum(
        frame: &TinyFrame,
        group_keys: Vec<String>,
        value_column: String,
    ) -> PyResult<TinyFrame> {
        if group_keys.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Group keys cannot be empty",
            ));
        }

        Self::validate_group_key_columns(frame, &group_keys)?;

        // Get the value column
        let value_col = frame.columns.get(&value_column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                "Column '{}' not found",
                value_column
            ))
        })?;

        // Create groups in parallel
        let groups: HashMap<GroupKeyRow, Vec<usize>> = (0..frame.length)
            .into_par_iter()
            .map(|i| {
                let key = Self::build_group_key_row(frame, &group_keys, i);
                (key, i)
            })
            .fold(HashMap::new, |mut acc, (key, idx)| {
                acc.entry(key).or_insert_with(Vec::new).push(idx);
                acc
            })
            .reduce(HashMap::new, |mut acc, mut other| {
                for (k, v) in other.drain() {
                    acc.entry(k).or_insert_with(Vec::new).extend(v);
                }
                acc
            });

        // Calculate sums in parallel
        let results: Vec<(GroupKeyRow, f64)> = groups
            .par_iter()
            .map(|(key, indices)| {
                let sum = Self::calculate_sum_for_indices(value_col, indices);
                (key.clone(), sum)
            })
            .collect();

        // Create result frame
        Self::create_result_frame(results, group_keys, value_column)
    }

    // Parallel GroupBy mean
    pub fn parallel_groupby_mean(
        frame: &TinyFrame,
        group_keys: Vec<String>,
        value_column: String,
    ) -> PyResult<TinyFrame> {
        if group_keys.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Group keys cannot be empty",
            ));
        }

        Self::validate_group_key_columns(frame, &group_keys)?;

        let value_col = frame.columns.get(&value_column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                "Column '{}' not found",
                value_column
            ))
        })?;

        let groups: HashMap<GroupKeyRow, Vec<usize>> = (0..frame.length)
            .into_par_iter()
            .map(|i| {
                let key = Self::build_group_key_row(frame, &group_keys, i);
                (key, i)
            })
            .fold(HashMap::new, |mut acc, (key, idx)| {
                acc.entry(key).or_insert_with(Vec::new).push(idx);
                acc
            })
            .reduce(HashMap::new, |mut acc, mut other| {
                for (k, v) in other.drain() {
                    acc.entry(k).or_insert_with(Vec::new).extend(v);
                }
                acc
            });

        let results: Vec<(GroupKeyRow, f64)> = groups
            .par_iter()
            .map(|(key, indices)| {
                let mean = Self::calculate_mean_for_indices(value_col, indices);
                (key.clone(), mean)
            })
            .collect();

        Self::create_result_frame(results, group_keys, value_column)
    }

    // Parallel filtering
    pub fn parallel_filter(
        frame: &TinyFrame,
        column: String,
        condition: String,
        value: &PyAny,
    ) -> PyResult<TinyFrame> {
        let col = frame.columns.get(&column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column))
        })?;

        // Create filter condition
        let filter_condition = FilterCondition::new(column, condition, value)?;

        // Apply filter in parallel
        let mask: Vec<bool> = (0..frame.length)
            .into_par_iter()
            .map(|i| match Self::get_value_at_index(col, i) {
                Some(val) => filter_condition.evaluate(&val),
                None => false,
            })
            .collect();

        // Apply mask to create filtered frame
        Self::apply_mask(frame, &mask)
    }

    // Parallel sorting
    pub fn parallel_sort(
        frame: &TinyFrame,
        by: Vec<String>,
        ascending: Option<bool>,
    ) -> PyResult<TinyFrame> {
        if by.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Sort columns cannot be empty",
            ));
        }

        // Validate columns exist
        for col_name in &by {
            if !frame.columns.contains_key(col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Column '{}' not found",
                    col_name
                )));
            }
        }

        if frame.length == 0 {
            return Ok(frame.clone());
        }

        // Create sort keys in parallel (fixed arity: one optional value per sort column)
        let sort_keys: Vec<GroupKeyRow> = (0..frame.length)
            .into_par_iter()
            .map(|i| Self::build_group_key_row(frame, &by, i))
            .collect();

        // Sort indices in parallel
        let mut indices: Vec<usize> = (0..frame.length).collect();
        indices.par_sort_by(|&a, &b| {
            Self::compare_sort_keys(&sort_keys[a], &sort_keys[b], ascending.unwrap_or(true))
        });

        // Create sorted frame
        Self::create_sorted_frame(frame, &indices)
    }

    // Helper methods
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
            TinyColumn::OptStr(v) => v
                .get(index)
                .and_then(|val| val.as_ref().map(|s| ValueEnum::Str(s.clone()))),
            TinyColumn::OptBool(v) => v.get(index).and_then(|val| val.map(ValueEnum::Bool)),
            TinyColumn::OptPyObject(v) => {
                v.get(index).and_then(|val| val.map(ValueEnum::PyObjectId))
            }
            TinyColumn::OptMixed(v) => v.get(index).and_then(|val| val.clone()),
        }
    }

    fn calculate_sum_for_indices(col: &TinyColumn, indices: &[usize]) -> f64 {
        match col {
            TinyColumn::Int(v) => indices.iter().map(|&i| v[i] as f64).sum(),
            TinyColumn::Float(v) => indices.iter().map(|&i| v[i]).sum(),
            TinyColumn::OptInt(v) => indices
                .iter()
                .filter_map(|&i| v[i].map(|val| val as f64))
                .sum(),
            TinyColumn::OptFloat(v) => indices.iter().filter_map(|&i| v[i]).sum(),
            _ => 0.0, // Unsupported types
        }
    }

    fn calculate_mean_for_indices(col: &TinyColumn, indices: &[usize]) -> f64 {
        let sum = Self::calculate_sum_for_indices(col, indices);
        let count = indices.len() as f64;
        if count > 0.0 {
            sum / count
        } else {
            0.0
        }
    }

    fn create_result_frame(
        results: Vec<(GroupKeyRow, f64)>,
        group_keys: Vec<String>,
        value_column: String,
    ) -> PyResult<TinyFrame> {
        if results.is_empty() {
            return Ok(TinyFrame::new());
        }

        let mut columns: HashMap<String, TinyColumn> = HashMap::new();
        let length = results.len();

        // Create group key columns
        for (i, key_name) in group_keys.iter().enumerate() {
            let mut values: Vec<Option<ValueEnum>> = Vec::with_capacity(results.len());
            for (key, _) in &results {
                values.push(key[i].clone());
            }
            columns.insert(key_name.clone(), TinyColumn::OptMixed(values));
        }

        // Create value column
        let value_values: Vec<f64> = results.iter().map(|(_, val)| *val).collect();
        columns.insert(value_column, TinyColumn::Float(value_values));

        Ok(TinyFrame {
            columns,
            length,
            py_objects: HashMap::new(),
        })
    }

    fn apply_mask(frame: &TinyFrame, mask: &[bool]) -> PyResult<TinyFrame> {
        let mut new_columns: HashMap<String, TinyColumn> = HashMap::new();
        let new_length = mask.iter().filter(|&&x| x).count();

        for (col_name, col_data) in &frame.columns {
            let new_col = Self::filter_column(col_data, mask)?;
            new_columns.insert(col_name.clone(), new_col);
        }

        Ok(TinyFrame {
            columns: new_columns,
            length: new_length,
            py_objects: frame.py_objects.clone(),
        })
    }

    fn filter_column(col: &TinyColumn, mask: &[bool]) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(v) => {
                let new_v: Vec<i64> = v
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Int(new_v))
            }
            TinyColumn::Float(v) => {
                let new_v: Vec<f64> = v
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Float(new_v))
            }
            TinyColumn::Str(v) => {
                let new_v: Vec<String> = v
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| val.clone())
                    .collect();
                Ok(TinyColumn::Str(new_v))
            }
            TinyColumn::Bool(v) => {
                let new_v: Vec<bool> = v
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask[*i])
                    .map(|(_, val)| *val)
                    .collect();
                Ok(TinyColumn::Bool(new_v))
            }
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported column type for parallel filtering",
            )),
        }
    }

    fn compare_sort_keys(a: &GroupKeyRow, b: &GroupKeyRow, ascending: bool) -> std::cmp::Ordering {
        for (val_a, val_b) in a.iter().zip(b.iter()) {
            let cmp = Self::compare_optional_values(val_a, val_b);
            if cmp != std::cmp::Ordering::Equal {
                return if ascending { cmp } else { cmp.reverse() };
            }
        }
        std::cmp::Ordering::Equal
    }

    /// `None` orders before `Some` so null/missing keys sort consistently.
    fn compare_optional_values(a: &Option<ValueEnum>, b: &Option<ValueEnum>) -> std::cmp::Ordering {
        match (a, b) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(va), Some(vb)) => Self::compare_values(va, vb),
        }
    }

    fn compare_values(a: &ValueEnum, b: &ValueEnum) -> std::cmp::Ordering {
        match (a, b) {
            (ValueEnum::Int(x), ValueEnum::Int(y)) => x.cmp(y),
            (ValueEnum::Float(x), ValueEnum::Float(y)) => {
                x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
            }
            (ValueEnum::Str(x), ValueEnum::Str(y)) => x.cmp(y),
            (ValueEnum::Bool(x), ValueEnum::Bool(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn create_sorted_frame(frame: &TinyFrame, indices: &[usize]) -> PyResult<TinyFrame> {
        let mut new_columns: HashMap<String, TinyColumn> = HashMap::new();

        for (col_name, col_data) in &frame.columns {
            let new_col = Self::sort_column(col_data, indices)?;
            new_columns.insert(col_name.clone(), new_col);
        }

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    fn sort_column(col: &TinyColumn, indices: &[usize]) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(v) => {
                let new_v: Vec<i64> = indices.iter().map(|&i| v[i]).collect();
                Ok(TinyColumn::Int(new_v))
            }
            TinyColumn::Float(v) => {
                let new_v: Vec<f64> = indices.iter().map(|&i| v[i]).collect();
                Ok(TinyColumn::Float(new_v))
            }
            TinyColumn::Str(v) => {
                let new_v: Vec<String> = indices.iter().map(|&i| v[i].clone()).collect();
                Ok(TinyColumn::Str(new_v))
            }
            TinyColumn::Bool(v) => {
                let new_v: Vec<bool> = indices.iter().map(|&i| v[i]).collect();
                Ok(TinyColumn::Bool(new_v))
            }
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported column type for parallel sorting",
            )),
        }
    }
}

// Filter condition for parallel filtering
pub struct FilterCondition {
    column: String,
    condition: String,
    value: ValueEnum,
}

impl FilterCondition {
    pub fn new(column: String, condition: String, value: &PyAny) -> PyResult<Self> {
        let value_enum = Self::convert_py_value(value)?;
        Ok(Self {
            column,
            condition,
            value: value_enum,
        })
    }

    pub fn evaluate(&self, val: &ValueEnum) -> bool {
        match self.condition.as_str() {
            "==" => self.compare_values(val, &self.value) == std::cmp::Ordering::Equal,
            "!=" => self.compare_values(val, &self.value) != std::cmp::Ordering::Equal,
            ">" => self.compare_values(val, &self.value) == std::cmp::Ordering::Greater,
            "<" => self.compare_values(val, &self.value) == std::cmp::Ordering::Less,
            ">=" => self.compare_values(val, &self.value) != std::cmp::Ordering::Less,
            "<=" => self.compare_values(val, &self.value) != std::cmp::Ordering::Greater,
            _ => false,
        }
    }

    fn compare_values(&self, a: &ValueEnum, b: &ValueEnum) -> std::cmp::Ordering {
        match (a, b) {
            (ValueEnum::Int(x), ValueEnum::Int(y)) => x.cmp(y),
            (ValueEnum::Float(x), ValueEnum::Float(y)) => {
                x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
            }
            (ValueEnum::Str(x), ValueEnum::Str(y)) => x.cmp(y),
            (ValueEnum::Bool(x), ValueEnum::Bool(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn convert_py_value(value: &PyAny) -> PyResult<ValueEnum> {
        if let Ok(int_val) = value.extract::<i64>() {
            Ok(ValueEnum::Int(int_val))
        } else if let Ok(float_val) = value.extract::<f64>() {
            Ok(ValueEnum::Float(float_val))
        } else if let Ok(str_val) = value.extract::<String>() {
            Ok(ValueEnum::Str(str_val))
        } else if let Ok(bool_val) = value.extract::<bool>() {
            Ok(ValueEnum::Bool(bool_val))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported value type for filtering",
            ))
        }
    }
}

#[cfg(test)]
mod parallel_groupby_tests {
    use super::ParallelOps;
    use crate::frame::{TinyColumn, TinyFrame, ValueEnum};
    use std::collections::HashMap;

    fn col_lens(frame: &TinyFrame) -> Vec<(String, usize)> {
        let mut v: Vec<_> = frame
            .columns
            .iter()
            .map(|(n, c)| (n.clone(), c.len()))
            .collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    #[test]
    fn parallel_groupby_sum_preserves_null_vs_some_second_key() {
        let mut columns = HashMap::new();
        columns.insert("k1".into(), TinyColumn::Str(vec!["A".into(), "A".into()]));
        columns.insert("k2".into(), TinyColumn::OptInt(vec![None, Some(1)]));
        columns.insert("v".into(), TinyColumn::Float(vec![10.0, 20.0]));
        let frame = TinyFrame {
            columns,
            length: 2,
            py_objects: HashMap::new(),
        };

        let out =
            ParallelOps::parallel_groupby_sum(&frame, vec!["k1".into(), "k2".into()], "v".into())
                .expect("parallel_groupby_sum");

        assert_eq!(out.length, 2, "distinct key rows should not collapse");
        let lens: Vec<usize> = out.columns.values().map(|c| c.len()).collect();
        assert!(
            lens.iter().all(|&l| l == 2),
            "all columns same length: {:?}",
            col_lens(&out)
        );
    }

    #[test]
    fn parallel_groupby_mean_chunked_merge_groupby_matches_row_count() {
        let mut columns = HashMap::new();
        columns.insert("k1".into(), TinyColumn::Str(vec!["X".into(), "X".into()]));
        columns.insert("k2".into(), TinyColumn::OptInt(vec![Some(1), None]));
        columns.insert("v".into(), TinyColumn::Float(vec![1.0, 2.0]));
        let frame = TinyFrame {
            columns,
            length: 2,
            py_objects: HashMap::new(),
        };

        use crate::chunked::ChunkedProcessor;
        let processor = ChunkedProcessor::new(1);
        let merged = processor
            .chunked_groupby_sum(&frame, vec!["k1".into(), "k2".into()], "v".into())
            .expect("chunked_groupby_sum");

        assert_eq!(merged.length, 2);
        let lens: Vec<usize> = merged.columns.values().map(|c| c.len()).collect();
        assert!(
            lens.iter().all(|&l| l == 2),
            "merged columns aligned: {:?}",
            col_lens(&merged)
        );

        let k2 = merged.columns.get("k2").expect("k2");
        match k2 {
            TinyColumn::OptMixed(vals) => {
                assert!(vals.iter().any(|o| *o == Some(ValueEnum::Int(1))));
                assert!(vals.iter().any(|o| o.is_none()));
            }
            _ => panic!("expected OptMixed k2 column"),
        }
    }
}
