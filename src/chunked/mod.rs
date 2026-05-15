use std::collections::HashMap;
use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn, ValueEnum};
use crate::parallel::{GroupKeyRow, ParallelOps};

// Chunked processing for large datasets
pub struct ChunkedProcessor {
    chunk_size: usize,
}

impl ChunkedProcessor {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    // Process large dataset in chunks
    pub fn process_large_dataset<F>(
        &self,
        frame: &TinyFrame,
        processor: F,
    ) -> PyResult<TinyFrame>
    where
        F: Fn(&TinyFrame) -> PyResult<TinyFrame> + Send + Sync,
    {
        if frame.length <= self.chunk_size {
            return processor(frame);
        }

        let mut results = Vec::new();
        let num_chunks = (frame.length + self.chunk_size - 1) / self.chunk_size;

        for chunk_idx in 0..num_chunks {
            let start = chunk_idx * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size, frame.length);
            
            let chunk = self.create_chunk(frame, start, end)?;
            let processed_chunk = processor(&chunk)?;
            results.push(processed_chunk);
        }

        self.merge_chunks(results)
    }

    // Chunked GroupBy operations
    pub fn chunked_groupby_sum(
        &self,
        frame: &TinyFrame,
        group_keys: Vec<String>,
        value_column: String,
    ) -> PyResult<TinyFrame> {
        if frame.length <= self.chunk_size {
            return ParallelOps::parallel_groupby_sum(frame, group_keys, value_column);
        }

        // Process each chunk
        let mut chunk_results = Vec::new();
        let num_chunks = (frame.length + self.chunk_size - 1) / self.chunk_size;

        for chunk_idx in 0..num_chunks {
            let start = chunk_idx * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size, frame.length);
            
            let chunk = self.create_chunk(frame, start, end)?;
            let chunk_result = ParallelOps::parallel_groupby_sum(&chunk, group_keys.clone(), value_column.clone())?;
            chunk_results.push(chunk_result);
        }

        // Merge results from all chunks
        self.merge_groupby_results(chunk_results, group_keys, value_column)
    }

    // Chunked filtering
    pub fn chunked_filter(
        &self,
        frame: &TinyFrame,
        column: String,
        condition: String,
        value: &PyAny,
    ) -> PyResult<TinyFrame> {
        if frame.length <= self.chunk_size {
            return ParallelOps::parallel_filter(frame, column, condition, value);
        }

        let mut results = Vec::new();
        let num_chunks = (frame.length + self.chunk_size - 1) / self.chunk_size;

        for chunk_idx in 0..num_chunks {
            let start = chunk_idx * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size, frame.length);
            
            let chunk = self.create_chunk(frame, start, end)?;
            let filtered_chunk = ParallelOps::parallel_filter(&chunk, column.clone(), condition.clone(), value)?;
            
            if filtered_chunk.length > 0 {
                results.push(filtered_chunk);
            }
        }

        if results.is_empty() {
            Ok(TinyFrame::new())
        } else {
            self.merge_chunks(results)
        }
    }

    // Chunked sorting
    pub fn chunked_sort(
        &self,
        frame: &TinyFrame,
        by: Vec<String>,
        ascending: Option<bool>,
    ) -> PyResult<TinyFrame> {
        if frame.length <= self.chunk_size {
            return ParallelOps::parallel_sort(frame, by, ascending);
        }

        // Sort each chunk individually
        let mut sorted_chunks = Vec::new();
        let num_chunks = (frame.length + self.chunk_size - 1) / self.chunk_size;

        for chunk_idx in 0..num_chunks {
            let start = chunk_idx * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size, frame.length);
            
            let chunk = self.create_chunk(frame, start, end)?;
            let sorted_chunk = ParallelOps::parallel_sort(&chunk, by.clone(), ascending)?;
            sorted_chunks.push(sorted_chunk);
        }

        // Merge sorted chunks using k-way merge
        self.k_way_merge(sorted_chunks, by, ascending)
    }

    // Streaming aggregations
    pub fn streaming_aggregation<F, R>(
        &self,
        frame: &TinyFrame,
        group_keys: Vec<String>,
        aggregator: F,
    ) -> PyResult<HashMap<GroupKeyRow, R>>
    where
        F: Fn(&[usize], &TinyFrame) -> PyResult<R> + Send + Sync,
        R: Clone + Send + Sync,
    {
        ParallelOps::validate_group_key_columns(frame, &group_keys)?;

        let mut global_groups: HashMap<GroupKeyRow, Vec<usize>> = HashMap::new();
        let num_chunks = (frame.length + self.chunk_size - 1) / self.chunk_size;

        for chunk_idx in 0..num_chunks {
            let start = chunk_idx * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size, frame.length);
            
            let chunk = self.create_chunk(frame, start, end)?;
            
            for i in 0..chunk.length {
                let key = ParallelOps::build_group_key_row(&chunk, &group_keys, i);
                global_groups.entry(key).or_insert_with(Vec::new).push(start + i);
            }
        }

        let mut results = HashMap::new();
        for (key, indices) in global_groups {
            let result = aggregator(&indices, frame)?;
            results.insert(key, result);
        }

        Ok(results)
    }

    // Helper methods
    fn create_chunk(&self, frame: &TinyFrame, start: usize, end: usize) -> PyResult<TinyFrame> {
        let mut chunk_columns: HashMap<String, TinyColumn> = HashMap::new();
        let chunk_length = end - start;

        for (col_name, col_data) in &frame.columns {
            let chunk_col = self.chunk_column(col_data, start, end)?;
            chunk_columns.insert(col_name.clone(), chunk_col);
        }

        Ok(TinyFrame {
            columns: chunk_columns,
            length: chunk_length,
            py_objects: frame.py_objects.clone(),
        })
    }

    fn chunk_column(&self, col: &TinyColumn, start: usize, end: usize) -> PyResult<TinyColumn> {
        match col {
            TinyColumn::Int(v) => {
                let chunk: Vec<i64> = v[start..end].to_vec();
                Ok(TinyColumn::Int(chunk))
            },
            TinyColumn::Float(v) => {
                let chunk: Vec<f64> = v[start..end].to_vec();
                Ok(TinyColumn::Float(chunk))
            },
            TinyColumn::Str(v) => {
                let chunk: Vec<String> = v[start..end].to_vec();
                Ok(TinyColumn::Str(chunk))
            },
            TinyColumn::Bool(v) => {
                let chunk: Vec<bool> = v[start..end].to_vec();
                Ok(TinyColumn::Bool(chunk))
            },
            TinyColumn::PyObject(v) => {
                let chunk: Vec<u64> = v[start..end].to_vec();
                Ok(TinyColumn::PyObject(chunk))
            },
            TinyColumn::Mixed(v) => {
                let chunk: Vec<ValueEnum> = v[start..end].to_vec();
                Ok(TinyColumn::Mixed(chunk))
            },
            TinyColumn::OptInt(v) => {
                let chunk: Vec<Option<i64>> = v[start..end].to_vec();
                Ok(TinyColumn::OptInt(chunk))
            },
            TinyColumn::OptFloat(v) => {
                let chunk: Vec<Option<f64>> = v[start..end].to_vec();
                Ok(TinyColumn::OptFloat(chunk))
            },
            TinyColumn::OptStr(v) => {
                let chunk: Vec<Option<String>> = v[start..end].to_vec();
                Ok(TinyColumn::OptStr(chunk))
            },
            TinyColumn::OptBool(v) => {
                let chunk: Vec<Option<bool>> = v[start..end].to_vec();
                Ok(TinyColumn::OptBool(chunk))
            },
            TinyColumn::OptPyObject(v) => {
                let chunk: Vec<Option<u64>> = v[start..end].to_vec();
                Ok(TinyColumn::OptPyObject(chunk))
            },
            TinyColumn::OptMixed(v) => {
                let chunk: Vec<Option<ValueEnum>> = v[start..end].to_vec();
                Ok(TinyColumn::OptMixed(chunk))
            },
        }
    }

    fn merge_chunks(&self, chunks: Vec<TinyFrame>) -> PyResult<TinyFrame> {
        if chunks.is_empty() {
            return Ok(TinyFrame::new());
        }

        if chunks.len() == 1 {
            return Ok(chunks.into_iter().next().unwrap());
        }

        let total_length: usize = chunks.iter().map(|c| c.length).sum();
        let mut merged_columns: HashMap<String, TinyColumn> = HashMap::new();

        // Get all column names from the first chunk
        let column_names: Vec<String> = chunks[0].columns.keys().cloned().collect();

        for col_name in column_names {
            let merged_col = self.merge_column(&chunks, &col_name)?;
            merged_columns.insert(col_name, merged_col);
        }

        Ok(TinyFrame {
            columns: merged_columns,
            length: total_length,
            py_objects: Self::merge_py_object_maps(&chunks),
        })
    }

    fn merge_py_object_maps(chunks: &[TinyFrame]) -> HashMap<u64, PyObject> {
        let mut merged = HashMap::new();
        for chunk in chunks {
            for (&id, obj) in &chunk.py_objects {
                merged.entry(id).or_insert_with(|| obj.clone());
            }
        }
        merged
    }

    fn merge_column(&self, chunks: &[TinyFrame], col_name: &str) -> PyResult<TinyColumn> {
        let first_chunk = &chunks[0];
        let first_col = first_chunk.columns.get(col_name)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", col_name)
            ))?;

        match first_col {
            TinyColumn::Int(_) => {
                let mut merged: Vec<i64> = Vec::new();
                for chunk in chunks {
                    let col = chunk.columns.get(col_name).ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                            "Column '{}' not found in chunk during merge",
                            col_name
                        ))
                    })?;
                    match col {
                        TinyColumn::Int(v) => merged.extend_from_slice(v),
                        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Column '{}' has inconsistent type across chunks",
                            col_name
                        ))),
                    }
                }
                Ok(TinyColumn::Int(merged))
            },
            TinyColumn::Float(_) => {
                let mut merged: Vec<f64> = Vec::new();
                for chunk in chunks {
                    let col = chunk.columns.get(col_name).ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                            "Column '{}' not found in chunk during merge",
                            col_name
                        ))
                    })?;
                    match col {
                        TinyColumn::Float(v) => merged.extend_from_slice(v),
                        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Column '{}' has inconsistent type across chunks",
                            col_name
                        ))),
                    }
                }
                Ok(TinyColumn::Float(merged))
            },
            TinyColumn::Str(_) => {
                let mut merged: Vec<String> = Vec::new();
                for chunk in chunks {
                    let col = chunk.columns.get(col_name).ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                            "Column '{}' not found in chunk during merge",
                            col_name
                        ))
                    })?;
                    match col {
                        TinyColumn::Str(v) => merged.extend(v.iter().cloned()),
                        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Column '{}' has inconsistent type across chunks",
                            col_name
                        ))),
                    }
                }
                Ok(TinyColumn::Str(merged))
            },
            TinyColumn::Bool(_) => {
                let mut merged: Vec<bool> = Vec::new();
                for chunk in chunks {
                    let col = chunk.columns.get(col_name).ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                            "Column '{}' not found in chunk during merge",
                            col_name
                        ))
                    })?;
                    match col {
                        TinyColumn::Bool(v) => merged.extend_from_slice(v),
                        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Column '{}' has inconsistent type across chunks",
                            col_name
                        ))),
                    }
                }
                Ok(TinyColumn::Bool(merged))
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Unsupported column type for chunk merging"
            )),
        }
    }

    fn merge_groupby_results(
        &self,
        chunk_results: Vec<TinyFrame>,
        group_keys: Vec<String>,
        value_column: String,
    ) -> PyResult<TinyFrame> {
        if chunk_results.is_empty() {
            return Ok(TinyFrame::new());
        }

        let mut aggregated: HashMap<GroupKeyRow, f64> = HashMap::new();

        for chunk in chunk_results {
            ParallelOps::validate_group_key_columns(&chunk, &group_keys)?;
            if !chunk.columns.contains_key(&value_column) {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                    "Column '{}' not found",
                    value_column
                )));
            }

            for i in 0..chunk.length {
                let key = ParallelOps::build_group_key_row(&chunk, &group_keys, i);

                if let Some(TinyColumn::Float(values)) = chunk.columns.get(&value_column) {
                    let value = values[i];
                    *aggregated.entry(key).or_insert(0.0) += value;
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                        "Value column '{}' must be Float for chunked groupby merge",
                        value_column
                    )));
                }
            }
        }

        let mut columns: HashMap<String, TinyColumn> = HashMap::new();
        let length = aggregated.len();

        for (i, key_name) in group_keys.iter().enumerate() {
            let mut values: Vec<Option<ValueEnum>> = Vec::with_capacity(aggregated.len());
            for (key, _) in &aggregated {
                values.push(key[i].clone());
            }
            columns.insert(key_name.clone(), TinyColumn::OptMixed(values));
        }

        let value_values: Vec<f64> = aggregated.values().cloned().collect();
        columns.insert(value_column, TinyColumn::Float(value_values));

        Ok(TinyFrame {
            columns,
            length,
            py_objects: HashMap::new(),
        })
    }

    fn k_way_merge(
        &self,
        mut sorted_chunks: Vec<TinyFrame>,
        by: Vec<String>,
        ascending: Option<bool>,
    ) -> PyResult<TinyFrame> {
        if sorted_chunks.is_empty() {
            return Ok(TinyFrame::new());
        }

        if sorted_chunks.len() == 1 {
            return Ok(sorted_chunks.pop().unwrap());
        }

        // Simple implementation: concatenate and re-sort
        // In a production system, this would use a proper k-way merge algorithm
        let mut all_chunks = Vec::new();
        for chunk in sorted_chunks {
            if chunk.length > 0 {
                all_chunks.push(chunk);
            }
        }

        if all_chunks.is_empty() {
            return Ok(TinyFrame::new());
        }

        let merged = self.merge_chunks(all_chunks)?;
        ParallelOps::parallel_sort(&merged, by, ascending)
    }
}

// Memory-efficient streaming processor
pub struct StreamingProcessor {
    max_memory_mb: usize,
}

impl StreamingProcessor {
    pub fn new(max_memory_mb: usize) -> Self {
        Self { max_memory_mb }
    }

    pub fn estimate_memory_usage(frame: &TinyFrame) -> usize {
        let mut total_bytes = 0;
        
        for col in frame.columns.values() {
            total_bytes += match col {
                TinyColumn::Int(v) => v.len() * std::mem::size_of::<i64>(),
                TinyColumn::Float(v) => v.len() * std::mem::size_of::<f64>(),
                TinyColumn::Str(v) => v.iter().map(|s| s.len()).sum::<usize>() + v.len() * std::mem::size_of::<String>(),
                TinyColumn::Bool(v) => v.len() * std::mem::size_of::<bool>(),
                TinyColumn::PyObject(v) => v.len() * std::mem::size_of::<u64>(),
                TinyColumn::Mixed(v) => v.len() * std::mem::size_of::<Option<ValueEnum>>(),
                TinyColumn::OptInt(v) => v.len() * std::mem::size_of::<Option<i64>>(),
                TinyColumn::OptFloat(v) => v.len() * std::mem::size_of::<Option<f64>>(),
                TinyColumn::OptStr(v) => v.iter().map(|s| s.as_ref().map(|s| s.len()).unwrap_or(0)).sum::<usize>() + v.len() * std::mem::size_of::<Option<String>>(),
                TinyColumn::OptBool(v) => v.len() * std::mem::size_of::<Option<bool>>(),
                TinyColumn::OptPyObject(v) => v.len() * std::mem::size_of::<Option<u64>>(),
                TinyColumn::OptMixed(v) => v.len() * std::mem::size_of::<Option<ValueEnum>>(),
            };
        }
        
        total_bytes
    }

    pub fn should_use_chunked_processing(&self, frame: &TinyFrame) -> bool {
        let memory_usage_mb = Self::estimate_memory_usage(frame) / (1024 * 1024);
        memory_usage_mb > self.max_memory_mb
    }

    pub fn get_optimal_chunk_size(&self, frame: &TinyFrame) -> usize {
        let memory_usage = Self::estimate_memory_usage(frame);
        let target_chunk_memory = self.max_memory_mb * 1024 * 1024;
        
        if memory_usage <= target_chunk_memory {
            frame.length
        } else {
            (frame.length * target_chunk_memory) / memory_usage
        }
    }
}
