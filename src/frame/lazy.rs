use pyo3::prelude::*;
use std::sync::Arc;
use crate::frame::{TinyFrame, TinyColumn};
use crate::frame::optimize::FilterCondition;

// Lazy operation trait
pub trait LazyOperation: Send + Sync {
    fn execute(&self, frame: &TinyFrame) -> PyResult<TinyFrame>;
    fn description(&self) -> String;
    fn memory_usage(&self) -> usize;
}

// Lazy filter operation
pub struct LazyFilter {
    condition: FilterCondition,
}

impl LazyFilter {
    pub fn new(condition: FilterCondition) -> Self {
        Self { condition }
    }
}

impl LazyOperation for LazyFilter {
    fn execute(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        Python::with_gil(|py| frame.filter_optimized(py, &self.condition))
    }

    fn description(&self) -> String {
        format!("Filter({} {})", self.condition.column, self.condition.condition)
    }

    fn memory_usage(&self) -> usize {
        // Filter operations are memory-efficient as they only store the condition
        std::mem::size_of::<Self>()
    }
}

// Lazy sort operation
#[derive(Clone)]
pub struct SortKey {
    pub column: String,
    pub ascending: bool,
}

pub struct LazySort {
    keys: Vec<SortKey>,
}

impl LazySort {
    pub fn new(keys: Vec<SortKey>) -> Self {
        Self { keys }
    }
}

impl LazyOperation for LazySort {
    fn execute(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        let key_names: Vec<String> = self.keys.iter().map(|k| k.column.clone()).collect();
        // For now, use the first key's ascending value for all keys
        // In a full implementation, this would handle multiple sort directions
        let ascending = self.keys.first().map(|k| k.ascending).unwrap_or(true);
        frame.sort_values(key_names, Some(ascending))
    }

    fn description(&self) -> String {
        let key_descriptions: Vec<String> = self.keys.iter()
            .map(|k| format!("{}({})", k.column, if k.ascending { "asc" } else { "desc" }))
            .collect();
        format!("Sort({})", key_descriptions.join(", "))
    }

    fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() + self.keys.len() * std::mem::size_of::<SortKey>()
    }
}

// Lazy dropna operation
pub struct LazyDropna {
    column: String,
}

impl LazyDropna {
    pub fn new(column: String) -> Self {
        Self { column }
    }
}

impl LazyOperation for LazyDropna {
    fn execute(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        frame.dropna(self.column.clone())
    }

    fn description(&self) -> String {
        format!("Dropna({})", self.column)
    }

    fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() + self.column.len()
    }
}

// Lazy column selection operation
pub struct LazySelect {
    columns: Vec<String>,
}

impl LazySelect {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}

impl LazyOperation for LazySelect {
    fn execute(&self, frame: &TinyFrame) -> PyResult<TinyFrame> {
        let mut new_columns = std::collections::HashMap::new();
        
        for col_name in &self.columns {
            if let Some(col_data) = frame.columns.get(col_name) {
                new_columns.insert(col_name.clone(), col_data.clone());
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("Column '{}' not found", col_name)
                ));
            }
        }

        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    fn description(&self) -> String {
        format!("Select({})", self.columns.join(", "))
    }

    fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() + self.columns.iter().map(|s| s.len()).sum::<usize>()
    }
}

// Lazy frame that chains operations
pub struct LazyFrame {
    source: Arc<TinyFrame>,
    operations: Vec<Box<dyn LazyOperation>>,
}

impl LazyFrame {
    pub fn new(frame: TinyFrame) -> Self {
        Self {
            source: Arc::new(frame),
            operations: Vec::new(),
        }
    }

    pub fn filter(mut self, condition: FilterCondition) -> Self {
        self.operations.push(Box::new(LazyFilter::new(condition)));
        self
    }

    pub fn sort(mut self, keys: Vec<SortKey>) -> Self {
        self.operations.push(Box::new(LazySort::new(keys)));
        self
    }

    pub fn dropna(mut self, column: String) -> Self {
        self.operations.push(Box::new(LazyDropna::new(column)));
        self
    }

    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.operations.push(Box::new(LazySelect::new(columns)));
        self
    }

    pub fn collect(self) -> PyResult<TinyFrame> {
        let mut current = self.source.as_ref().clone();
        
        for operation in self.operations {
            current = operation.execute(&current)?;
        }
        
        Ok(current)
    }

    pub fn explain(&self) -> String {
        let mut explanation = "LazyFrame operations:\n".to_string();
        for (i, op) in self.operations.iter().enumerate() {
            explanation.push_str(&format!("  {}: {}\n", i + 1, op.description()));
        }
        explanation
    }

    pub fn memory_usage(&self) -> usize {
        let source_memory = self.source.columns.values().map(|col| col.len()).sum::<usize>();
        let operations_memory: usize = self.operations.iter().map(|op| op.memory_usage()).sum();
        source_memory + operations_memory
    }

    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// Lazy evaluation optimizer
pub struct LazyOptimizer;

impl LazyOptimizer {
    pub fn optimize(operations: Vec<Box<dyn LazyOperation>>) -> Vec<Box<dyn LazyOperation>> {
        // For now, return operations as-is
        // In a full implementation, this would combine compatible operations
        operations
    }
}

// Clone trait for LazyOperation
impl Clone for LazyFilter {
    fn clone(&self) -> Self {
        Self {
            condition: self.condition.clone(),
        }
    }
}

impl Clone for LazySort {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
        }
    }
}

impl Clone for LazyDropna {
    fn clone(&self) -> Self {
        Self {
            column: self.column.clone(),
        }
    }
}

impl Clone for LazySelect {
    fn clone(&self) -> Self {
        Self {
            columns: self.columns.clone(),
        }
    }
}
