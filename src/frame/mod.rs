use crate::column::TinyCol;
use pyo3::prelude::*;
use std::collections::HashMap;

pub mod cast;
pub mod convert;
pub mod edit;
pub mod fillna;
pub mod iter;
pub mod lazy;
pub mod optimize;
pub mod string_optimize;

mod column;
mod value;

pub use column::{TinyColumn, TinyColumnIter};
pub use value::ValueEnum;

/// TinyFrame
///
/// A fast, flexible DataFrame-like structure implemented in Rust for Python.
///
/// Supports type inference, optional and mixed columns, fillna, casting, editing, and row-wise iteration.
#[pyclass]
#[derive(Clone)]
pub struct TinyFrame {
    pub columns: HashMap<String, TinyColumn>,
    pub length: usize,
    pub py_objects: HashMap<u64, PyObject>,
}

#[pymethods]
impl TinyFrame {
    /// Create a new empty TinyFrame.
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        TinyFrame {
            columns: HashMap::new(),
            length: 0,
            py_objects: HashMap::new(),
        }
    }

    /// Create a TinyFrame from a list of Python dictionaries.
    ///
    /// Args:
    ///     records (List[dict]): List of Python dictionaries.
    ///
    /// Returns:
    ///     TinyFrame: New frame inferred from the records.
    #[staticmethod]
    #[pyo3(text_signature = "(records)")]
    pub fn from_dicts(py: Python, records: &PyAny) -> PyResult<Self> {
        convert::from_dicts_impl(py, records)
    }

    /// Convert the TinyFrame to a list of dictionaries.
    ///
    /// Returns:
    ///     List[dict]: Frame data as a list of dicts.
    fn to_dicts(&self, py: Python) -> PyResult<Vec<PyObject>> {
        convert::to_dicts_impl(self, py)
    }

    /// Fill missing (None) values in the frame.
    ///
    /// Args:
    ///     value (dict or scalar): Dictionary mapping column names to fill values or a single scalar value.
    fn fillna(&mut self, py: Python, value: &PyAny) -> PyResult<()> {
        fillna::fillna_impl(self, py, value)
    }

    /// Cast a column to a different type.
    ///
    /// Args:
    ///     column_name (str): Name of the column.
    ///     new_type (type): Target Python type (e.g., int, float, str, bool).
    fn cast_column(&mut self, py: Python, column_name: String, new_type: &PyAny) -> PyResult<()> {
        cast::cast_column_impl(self, py, column_name, new_type)
    }

    /// Edit a column using a custom Python function.  
    ///
    /// Args:
    ///     column_name (str): Name of the column.
    ///     func (callable): Python function to apply to each value.
    fn edit_column(&mut self, py: Python, column_name: String, func: PyObject) -> PyResult<()> {
        edit::edit_column_impl(self, py, column_name, func)
    }

    /// Drop specified columns from the frame.
    ///
    /// Args:
    ///     columns_to_drop (List[str]): List of column names to remove.
    fn drop_columns(&mut self, columns_to_drop: Vec<String>) -> PyResult<()> {
        for col_name in columns_to_drop {
            if !self.columns.contains_key(&col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Column '{}' not found",
                    col_name
                )));
            }
            self.columns.remove(&col_name);
        }
        Ok(())
    }

    /// Rename a column.
    ///
    /// Args:
    ///     old_name (str): Original column name.
    ///     new_name (str): New column name.
    fn rename_column(&mut self, old_name: String, new_name: String) -> PyResult<()> {
        if !self.columns.contains_key(&old_name) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                "Column '{}' not found",
                old_name
            )));
        }
        if self.columns.contains_key(&new_name) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Column '{}' already exists",
                new_name
            )));
        }
        let col = self.columns.remove(&old_name).unwrap();
        self.columns.insert(new_name, col);
        Ok(())
    }

    /// Filter rows based on a condition.
    ///
    /// Args:
    ///     column (str): Column name to filter on.
    ///     condition (str): Condition operator ('==', '!=', '>', '<', '>=', '<=', 'in', 'not_in').
    ///     value: Value to compare against.
    ///
    /// Returns:
    ///     TinyFrame: New frame with filtered rows.
    fn filter(
        &self,
        py: Python,
        column: String,
        condition: String,
        value: &PyAny,
    ) -> PyResult<Self> {
        let col = self.columns.get(&column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column))
        })?;

        let mut filtered_indices = Vec::new();

        for (idx, val) in col.iter().enumerate() {
            let matches = match condition.as_str() {
                "==" => match val {
                    ValueEnum::Str(_) => self.compare_string_values(&val, value, py, "==")?,
                    _ => self.compare_values(&val, value, py, |a, b| a == b)?,
                },
                "!=" => match val {
                    ValueEnum::Str(_) => self.compare_string_values(&val, value, py, "!=")?,
                    _ => self.compare_values(&val, value, py, |a, b| a != b)?,
                },
                ">" => self.compare_values(&val, value, py, |a, b| a > b)?,
                "<" => self.compare_values(&val, value, py, |a, b| a < b)?,
                ">=" => self.compare_values(&val, value, py, |a, b| a >= b)?,
                "<=" => self.compare_values(&val, value, py, |a, b| a <= b)?,
                "in" => self.check_in_values(&val, value, py)?,
                "not_in" => !self.check_in_values(&val, value, py)?,
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Unknown condition: {}",
                        condition
                    )))
                }
            };

            if matches {
                filtered_indices.push(idx);
            }
        }

        self.filter_by_indices(filtered_indices)
    }

    /// Filter rows where column values are not null.
    ///
    /// Args:
    ///     column (str): Column name to check for nulls.
    ///
    /// Returns:
    ///     TinyFrame: New frame with non-null rows.
    fn dropna(&self, column: String) -> PyResult<Self> {
        let col = self.columns.get(&column).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Column '{}' not found", column))
        })?;

        let mut filtered_indices = Vec::new();

        for idx in 0..self.length {
            let is_not_null = match col {
                TinyColumn::OptInt(v) => v[idx].is_some(),
                TinyColumn::OptFloat(v) => v[idx].is_some(),
                TinyColumn::OptStr(v) => v[idx].is_some(),
                TinyColumn::OptBool(v) => v[idx].is_some(),
                TinyColumn::OptMixed(v) => v[idx].is_some(),
                TinyColumn::OptPyObject(v) => v[idx].is_some(),
                _ => true, // Non-optional columns are never null
            };

            if is_not_null {
                filtered_indices.push(idx);
            }
        }

        self.filter_by_indices(filtered_indices)
    }

    /// Sort the frame by one or more columns.
    ///
    /// Args:
    ///     by (List[str]): Column names to sort by.
    ///     ascending (bool): Sort in ascending order (default True).
    ///
    /// Returns:
    ///     TinyFrame: New frame with sorted rows.
    fn sort_values(&self, by: Vec<String>, ascending: Option<bool>) -> PyResult<Self> {
        let ascending = ascending.unwrap_or(true);

        // Handle empty frame
        if self.length == 0 {
            return Ok(self.clone());
        }

        // Validate all columns exist
        for col_name in &by {
            if !self.columns.contains_key(col_name) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Column '{}' not found",
                    col_name
                )));
            }
        }

        let mut indices: Vec<usize> = (0..self.length).collect();

        // Sort by multiple columns (stable sort)
        indices.sort_by(|&a, &b| {
            for col_name in &by {
                let col = match self.columns.get(col_name) {
                    Some(col) => col,
                    None => {
                        // This should not happen as we check columns exist before sorting
                        return std::cmp::Ordering::Equal;
                    }
                };

                let val_a = self.get_value_at_index(col, a);
                let val_b = self.get_value_at_index(col, b);

                let comparison = self.compare_for_sort(val_a, val_b);
                if comparison != std::cmp::Ordering::Equal {
                    return if ascending {
                        comparison
                    } else {
                        comparison.reverse()
                    };
                }
            }
            std::cmp::Ordering::Equal
        });

        self.filter_by_indices(indices)
    }

    /// Return the number of rows.
    ///
    /// Returns:
    ///     int: Number of rows in the frame.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the frame is empty.
    ///
    /// Returns:
    ///     bool: True if empty, False otherwise.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Shape of the frame (rows, columns).
    ///
    /// Returns:
    ///     Tuple[int, int]: (number of rows, number of columns).
    #[getter]
    pub fn shape(&self) -> (usize, usize) {
        (self.length, self.columns.len())
    }

    /// Get column names.
    ///
    /// Returns:
    ///     List[str]: List of column names
    #[getter]
    pub fn columns(&self) -> Vec<String> {
        self.columns.keys().cloned().collect()
    }

    /// Return string representation of the frame.
    fn __repr__(&self) -> String {
        let mut col_strs = Vec::new();
        for (name, col) in &self.columns {
            let type_str = match col {
                TinyColumn::Int(_) => "Int",
                TinyColumn::Float(_) => "Float",
                TinyColumn::Str(_) => "Str",
                TinyColumn::Bool(_) => "Bool",
                TinyColumn::OptInt(_) => "OptInt",
                TinyColumn::OptFloat(_) => "OptFloat",
                TinyColumn::OptStr(_) => "OptStr",
                TinyColumn::OptBool(_) => "OptBool",
                TinyColumn::Mixed(_) => "Mixed",
                TinyColumn::OptMixed(_) => "OptMixed",
                TinyColumn::PyObject(_) => "PyObject",
                TinyColumn::OptPyObject(_) => "OptPyObject",
            };
            col_strs.push(format!("'{}': '{}'", name, type_str));
        }
        format!(
            "TinyFrame(rows={}, columns={}, cols={{ {} }})",
            self.length,
            col_strs.len(),
            col_strs.join(", ")
        )
    }

    /// Iterate over rows as dictionaries.
    fn __iter__(slf: PyRef<Self>) -> PyResult<crate::frame::iter::TinyFrameRowIter> {
        Ok(crate::frame::iter::TinyFrameRowIter::new(slf.into()))
    }

    fn col(&self, py: Python, name: String) -> PyResult<Py<TinyCol>> {
        if !self.columns.contains_key(&name) {
            return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                "Column '{}' not found",
                name
            )));
        }

        let frame_clone = self.clone();
        let py_frame = Py::new(py, frame_clone)?;

        let col = TinyCol {
            name,
            frame: py_frame,
        };
        Py::new(py, col)
    }

    /// Inner join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the inner join
    pub fn inner_join(
        &self,
        other: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<Self> {
        crate::joins::JoinOps::inner_join(self, other, left_on, right_on)
    }

    /// Left join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the left join
    pub fn left_join(
        &self,
        other: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<Self> {
        crate::joins::JoinOps::left_join(self, other, left_on, right_on)
    }

    /// Right join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the right join
    pub fn right_join(
        &self,
        other: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<Self> {
        crate::joins::JoinOps::right_join(self, other, left_on, right_on)
    }

    /// Outer join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///     left_on: List of column names from this frame to join on
    ///     right_on: List of column names from the other frame to join on
    ///
    /// Returns:
    ///     TinyFrame: The result of the outer join
    pub fn outer_join(
        &self,
        other: &TinyFrame,
        left_on: Vec<String>,
        right_on: Vec<String>,
    ) -> PyResult<Self> {
        crate::joins::JoinOps::outer_join(self, other, left_on, right_on)
    }

    /// Cross join with another TinyFrame.
    ///
    /// Args:
    ///     other: The other TinyFrame to join with
    ///
    /// Returns:
    ///     TinyFrame: The result of the cross join
    pub fn cross_join(&self, other: &TinyFrame) -> PyResult<Self> {
        crate::joins::JoinOps::cross_join(self, other)
    }

    /// Generate descriptive statistics for numeric columns
    ///
    /// Returns:
    ///     TinyFrame: A frame containing statistics (count, mean, std, min, 25%, 50%, 75%, max)
    pub fn describe(&self) -> PyResult<Self> {
        crate::stats::describe_impl(self)
    }

    /// Calculate correlation matrix for numeric columns
    ///
    /// Returns:
    ///     TinyFrame: Correlation matrix
    pub fn corr(&self) -> PyResult<Self> {
        crate::stats::corr_impl(self)
    }

    /// Calculate covariance matrix for numeric columns
    ///
    /// Returns:
    ///     TinyFrame: Covariance matrix
    pub fn cov(&self) -> PyResult<Self> {
        crate::stats::cov_impl(self)
    }

    /// Calculate correlation between two specific columns
    ///
    /// Args:
    ///     column1: First column name
    ///     column2: Second column name
    ///
    /// Returns:
    ///     f64: Correlation coefficient
    pub fn corr_with(&self, column1: String, column2: String) -> PyResult<f64> {
        crate::stats::corr_with_impl(self, &column1, &column2)
    }

    /// Calculate covariance between two specific columns
    ///
    /// Args:
    ///     column1: First column name
    ///     column2: Second column name
    ///
    /// Returns:
    ///     f64: Covariance value
    pub fn cov_with(&self, column1: String, column2: String) -> PyResult<f64> {
        crate::stats::cov_with_impl(self, &column1, &column2)
    }

    /// Calculate skewness for numeric columns
    ///
    /// Args:
    ///     column: Column name to calculate skewness for
    ///
    /// Returns:
    ///     f64: Skewness value
    pub fn skew(&self, column: String) -> PyResult<f64> {
        crate::stats::skew_impl(self, &column)
    }

    /// Calculate kurtosis for numeric columns
    ///
    /// Args:
    ///     column: Column name to calculate kurtosis for
    ///
    /// Returns:
    ///     f64: Kurtosis value
    pub fn kurtosis(&self, column: String) -> PyResult<f64> {
        crate::stats::kurtosis_impl(self, &column)
    }

    /// Calculate quantile for numeric columns
    ///
    /// Args:
    ///     column: Column name to calculate quantile for
    ///     q: Quantile value (0.0 to 1.0)
    ///
    /// Returns:
    ///     f64: Quantile value
    pub fn quantile(&self, column: String, q: f64) -> PyResult<f64> {
        crate::stats::quantile_impl(self, &column, q)
    }

    /// Calculate mode for any column type
    ///
    /// Args:
    ///     column: Column name to calculate mode for
    ///
    /// Returns:
    ///     PyObject: Most frequent value
    pub fn mode(&self, py: Python, column: String) -> PyResult<PyObject> {
        crate::stats::mode_impl(self, py, &column)
    }

    /// Count unique values in a column
    ///
    /// Args:
    ///     column: Column name to count unique values for
    ///
    /// Returns:
    ///     usize: Number of unique values
    pub fn nunique(&self, column: String) -> PyResult<usize> {
        crate::stats::nunique_impl(self, &column)
    }

    /// Convert datetime string column to timestamps
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with timestamp column added
    pub fn to_timestamps(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::to_timestamps_impl(self, &column)
    }

    /// Extract year from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with year column added
    pub fn dt_year(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_year_impl(self, &column)
    }

    /// Extract month from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with month column added
    pub fn dt_month(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_month_impl(self, &column)
    }

    /// Extract day from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with day column added
    pub fn dt_day(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_day_impl(self, &column)
    }

    /// Extract hour from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with hour column added
    pub fn dt_hour(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_hour_impl(self, &column)
    }

    /// Extract minute from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with minute column added
    pub fn dt_minute(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_minute_impl(self, &column)
    }

    /// Extract second from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with second column added
    pub fn dt_second(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_second_impl(self, &column)
    }

    /// Extract day of week from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with day of week column added
    pub fn dt_day_of_week(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_day_of_week_impl(self, &column)
    }

    /// Extract day of year from datetime string column
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with day of year column added
    pub fn dt_day_of_year(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_day_of_year_impl(self, &column)
    }

    /// Calculate time differences between consecutive datetime values
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with time difference column added
    pub fn dt_diff(&self, column: String) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_diff_impl(self, &column)
    }

    /// Shift datetime values by specified seconds
    ///
    /// Args:
    ///     column: Column name containing datetime strings
    ///     seconds: Number of seconds to shift by
    ///
    /// Returns:
    ///     TinyFrame: New frame with shifted datetime column added
    pub fn dt_shift(&self, column: String, seconds: i64) -> PyResult<Self> {
        crate::timeseries::TimeSeriesOps::dt_shift_impl(self, &column, seconds)
    }

    /// Calculate rolling mean for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///     window_size: Size of the rolling window
    ///
    /// Returns:
    ///     TinyFrame: New frame with rolling mean column added
    pub fn rolling_mean(&self, column: String, window_size: usize) -> PyResult<Self> {
        let window = crate::window::RollingWindow::new(window_size);
        crate::window::WindowOps::rolling_mean_impl(self, &column, window)
    }

    /// Calculate rolling sum for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///     window_size: Size of the rolling window
    ///
    /// Returns:
    ///     TinyFrame: New frame with rolling sum column added
    pub fn rolling_sum(&self, column: String, window_size: usize) -> PyResult<Self> {
        let window = crate::window::RollingWindow::new(window_size);
        crate::window::WindowOps::rolling_sum_impl(self, &column, window)
    }

    /// Calculate rolling standard deviation for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///     window_size: Size of the rolling window
    ///
    /// Returns:
    ///     TinyFrame: New frame with rolling std column added
    pub fn rolling_std(&self, column: String, window_size: usize) -> PyResult<Self> {
        let window = crate::window::RollingWindow::new(window_size);
        crate::window::WindowOps::rolling_std_impl(self, &column, window)
    }

    /// Calculate expanding mean for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///
    /// Returns:
    ///     TinyFrame: New frame with expanding mean column added
    pub fn expanding_mean(&self, column: String) -> PyResult<Self> {
        let window = crate::window::ExpandingWindow::new();
        crate::window::WindowOps::expanding_mean_impl(self, &column, window)
    }

    /// Calculate expanding sum for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///
    /// Returns:
    ///     TinyFrame: New frame with expanding sum column added
    pub fn expanding_sum(&self, column: String) -> PyResult<Self> {
        let window = crate::window::ExpandingWindow::new();
        crate::window::WindowOps::expanding_sum_impl(self, &column, window)
    }

    /// Calculate ranks for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///     method: Ranking method ("average", "min", "max", "first", "dense")
    ///
    /// Returns:
    ///     TinyFrame: New frame with rank column added
    pub fn rank(&self, column: String, method: String) -> PyResult<Self> {
        let rank_method = match method.as_str() {
            "average" => crate::ranking::RankMethod::Average,
            "min" => crate::ranking::RankMethod::Min,
            "max" => crate::ranking::RankMethod::Max,
            "first" => crate::ranking::RankMethod::First,
            "dense" => crate::ranking::RankMethod::Dense,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid ranking method. Must be one of: average, min, max, first, dense",
                ))
            }
        };
        crate::ranking::RankingOps::rank_impl(self, &column, rank_method)
    }

    /// Calculate percentage change for a numeric column
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///
    /// Returns:
    ///     TinyFrame: New frame with percentage change column added
    pub fn pct_change(&self, column: String) -> PyResult<Self> {
        crate::ranking::RankingOps::pct_change_impl(self, &column)
    }

    /// Convert strings to uppercase
    ///
    /// Args:
    ///     column: Column name containing string data
    ///
    /// Returns:
    ///     TinyFrame: New frame with uppercase string column added
    pub fn str_upper(&self, column: String) -> PyResult<Self> {
        crate::string::StringOps::str_upper_impl(self, &column)
    }

    /// Convert strings to lowercase
    ///
    /// Args:
    ///     column: Column name containing string data
    ///
    /// Returns:
    ///     TinyFrame: New frame with lowercase string column added
    pub fn str_lower(&self, column: String) -> PyResult<Self> {
        crate::string::StringOps::str_lower_impl(self, &column)
    }

    /// Strip whitespace from strings
    ///
    /// Args:
    ///     column: Column name containing string data
    ///
    /// Returns:
    ///     TinyFrame: New frame with stripped string column added
    pub fn str_strip(&self, column: String) -> PyResult<Self> {
        crate::string::StringOps::str_strip_impl(self, &column)
    }

    /// Replace substrings in strings
    ///
    /// Args:
    ///     column: Column name containing string data
    ///     from: Substring to replace
    ///     to: Replacement substring
    ///
    /// Returns:
    ///     TinyFrame: New frame with replaced string column added
    pub fn str_replace(&self, column: String, from: String, to: String) -> PyResult<Self> {
        crate::string::StringOps::str_replace_impl(self, &column, &from, &to)
    }

    /// Split strings by delimiter
    ///
    /// Args:
    ///     column: Column name containing string data
    ///     delimiter: Delimiter to split on
    ///
    /// Returns:
    ///     TinyFrame: New frame with split string column added
    pub fn str_split(&self, column: String, delimiter: String) -> PyResult<Self> {
        crate::string::StringOps::str_split_impl(self, &column, &delimiter)
    }

    /// Check if strings contain substring
    ///
    /// Args:
    ///     column: Column name containing string data
    ///     substring: Substring to search for
    ///
    /// Returns:
    ///     TinyFrame: New frame with boolean column indicating if substring is found
    pub fn str_contains(&self, column: String, substring: String) -> PyResult<Self> {
        crate::string::StringOps::str_contains_impl(self, &column, &substring)
    }

    /// Get string length
    ///
    /// Args:
    ///     column: Column name containing string data
    ///
    /// Returns:
    ///     TinyFrame: New frame with integer column containing string lengths
    pub fn str_len(&self, column: String) -> PyResult<Self> {
        crate::string::StringOps::str_len_impl(self, &column)
    }

    /// Concatenate strings
    ///
    /// Args:
    ///     column: Column name containing string data
    ///     separator: Separator to use between strings
    ///
    /// Returns:
    ///     TinyFrame: New frame with concatenated string column added
    pub fn str_cat(&self, column: String, separator: String) -> PyResult<Self> {
        crate::string::StringOps::str_cat_impl(self, &column, &separator)
    }

    /// Validate that all values in a column are not null
    ///
    /// Args:
    ///     column: Column name to validate
    ///
    /// Returns:
    ///     TinyFrame: New frame with boolean column indicating validation results
    pub fn validate_not_null(&self, column: String) -> PyResult<Self> {
        crate::validation::ValidationOps::validate_not_null_impl(self, &column)
    }

    /// Validate that all values in a numeric column are within a range
    ///
    /// Args:
    ///     column: Column name containing numeric data
    ///     min: Minimum value (optional)
    ///     max: Maximum value (optional)
    ///
    /// Returns:
    ///     TinyFrame: New frame with boolean column indicating validation results
    pub fn validate_range(
        &self,
        column: String,
        min: Option<f64>,
        max: Option<f64>,
    ) -> PyResult<Self> {
        crate::validation::ValidationOps::validate_range_impl(self, &column, min, max)
    }

    /// Validate that all values in a string column match a pattern
    ///
    /// Args:
    ///     column: Column name containing string data
    ///     pattern: Pattern to match
    ///
    /// Returns:
    ///     TinyFrame: New frame with boolean column indicating validation results
    pub fn validate_pattern(&self, column: String, pattern: String) -> PyResult<Self> {
        crate::validation::ValidationOps::validate_pattern_impl(self, &column, &pattern)
    }

    /// Validate that all values in a column are unique
    ///
    /// Args:
    ///     column: Column name to validate
    ///
    /// Returns:
    ///     TinyFrame: New frame with boolean column indicating validation results
    pub fn validate_unique(&self, column: String) -> PyResult<Self> {
        crate::validation::ValidationOps::validate_unique_impl(self, &column)
    }

    /// Get validation summary for a column
    ///
    /// Args:
    ///     column: Column name to summarize
    ///
    /// Returns:
    ///     dict: Dictionary containing validation statistics
    pub fn validation_summary(&self, column: String) -> PyResult<HashMap<String, f64>> {
        crate::validation::ValidationOps::validation_summary_impl(self, &column)
    }
}

impl TinyFrame {
    // Helper methods for filtering and sorting
    fn filter_by_indices(&self, indices: Vec<usize>) -> PyResult<Self> {
        let mut new_columns: HashMap<String, TinyColumn> = HashMap::new();

        for (name, column) in &self.columns {
            new_columns.insert(name.clone(), column.gather_rows(&indices));
        }

        Ok(TinyFrame {
            columns: new_columns,
            length: indices.len(),
            py_objects: self.py_objects.clone(),
        })
    }

    fn compare_values(
        &self,
        val: &ValueEnum,
        py_value: &PyAny,
        _py: Python,
        compare_fn: fn(f64, f64) -> bool,
    ) -> PyResult<bool> {
        match val {
            ValueEnum::Int(v) => {
                let py_f64 = py_value.extract::<f64>()?;
                Ok(compare_fn(*v as f64, py_f64))
            }
            ValueEnum::Float(v) => {
                let py_f64 = py_value.extract::<f64>()?;
                Ok(compare_fn(*v, py_f64))
            }
            ValueEnum::Str(_) => return Ok(false), // String comparison handled separately
            ValueEnum::Bool(_) => return Ok(false), // Bool comparison not implemented yet
            ValueEnum::PyObjectId(_) => return Ok(false), // PyObject comparison not implemented yet
        }
    }

    fn compare_string_values(
        &self,
        val: &ValueEnum,
        py_value: &PyAny,
        _py: Python,
        condition: &str,
    ) -> PyResult<bool> {
        match val {
            ValueEnum::Str(s) => {
                let py_str: String = py_value.extract()?;
                Ok(match condition {
                    "==" => s == &py_str,
                    "!=" => s != &py_str,
                    _ => false, // Other comparisons not supported for strings
                })
            }
            _ => Ok(false),
        }
    }

    fn check_in_values(&self, val: &ValueEnum, py_value: &PyAny, _py: Python) -> PyResult<bool> {
        // This is a simplified implementation - in practice, you'd want to handle different types
        match val {
            ValueEnum::Str(s) => {
                let py_str: String = py_value.extract()?;
                Ok(s == &py_str)
            }
            _ => Ok(false), // Other types not implemented yet
        }
    }

    fn get_value_at_index(&self, column: &TinyColumn, index: usize) -> Option<f64> {
        match column {
            TinyColumn::Int(v) => Some(v[index] as f64),
            TinyColumn::Float(v) => Some(v[index]),
            TinyColumn::OptInt(v) => v[index].map(|x| x as f64),
            TinyColumn::OptFloat(v) => v[index],
            _ => None,
        }
    }

    fn compare_for_sort(&self, val_a: Option<f64>, val_b: Option<f64>) -> std::cmp::Ordering {
        match (val_a, val_b) {
            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }
}
