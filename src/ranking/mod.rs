use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn};
use std::collections::HashMap;

/// Ranking method for rank function
#[derive(Clone, Debug)]
pub enum RankMethod {
    Average,
    Min,
    Max,
    First,
    Dense,
}

/// Ranking functions for TinyFrame
pub struct RankingOps;

impl RankingOps {
    /// Calculate ranks for a numeric column
    pub fn rank_impl(frame: &TinyFrame, column: &str, method: RankMethod) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut ranks = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                let indexed_values: Vec<(usize, f64)> = v.iter().enumerate().map(|(i, &val)| (i, val)).collect();
                let mut sorted_values = indexed_values.clone();
                sorted_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                
                ranks = Self::calculate_ranks(&sorted_values, &method, v.len());
            },
            TinyColumn::Int(v) => {
                let indexed_values: Vec<(usize, i64)> = v.iter().enumerate().map(|(i, &val)| (i, val)).collect();
                let mut sorted_values = indexed_values.clone();
                sorted_values.sort_by(|a, b| a.1.cmp(&b.1));
                
                // Convert to f64 for consistent ranking
                let float_values: Vec<(usize, f64)> = sorted_values.iter()
                    .map(|(idx, val)| (*idx, *val as f64))
                    .collect();
                ranks = Self::calculate_ranks(&float_values, &method, v.len());
            },
            TinyColumn::OptFloat(v) => {
                let indexed_values: Vec<(usize, Option<f64>)> = v.iter().enumerate().map(|(i, &val)| (i, val)).collect();
                let mut sorted_values = indexed_values.clone();
                sorted_values.sort_by(|a, b| {
                    match (a.1, b.1) {
                        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap(),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
                
                ranks = Self::calculate_ranks_optional(&sorted_values, &method, v.len());
            },
            TinyColumn::OptInt(v) => {
                let indexed_values: Vec<(usize, Option<i64>)> = v.iter().enumerate().map(|(i, &val)| (i, val)).collect();
                let mut sorted_values = indexed_values.clone();
                sorted_values.sort_by(|a, b| {
                    match (a.1, b.1) {
                        (Some(x), Some(y)) => x.cmp(&y),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
                
                // Convert to f64 for consistent ranking
                let float_values: Vec<(usize, Option<f64>)> = sorted_values.iter()
                    .map(|(idx, val)| (*idx, val.map(|v| v as f64)))
                    .collect();
                ranks = Self::calculate_ranks_optional(&float_values, &method, v.len());
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Ranking only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_rank", column), TinyColumn::OptFloat(ranks));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate percentage change for a numeric column
    pub fn pct_change_impl(frame: &TinyFrame, column: &str) -> PyResult<TinyFrame> {
        let col = frame.columns.get(column)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Column '{}' not found", column)
            ))?;

        let mut pct_changes = Vec::new();
        match col {
            TinyColumn::Float(v) => {
                pct_changes.push(None); // First value is always None
                for i in 1..v.len() {
                    if v[i-1] != 0.0 {
                        let pct_change = (v[i] - v[i-1]) / v[i-1] * 100.0;
                        pct_changes.push(Some(pct_change));
                    } else {
                        pct_changes.push(None);
                    }
                }
            },
            TinyColumn::Int(v) => {
                pct_changes.push(None); // First value is always None
                for i in 1..v.len() {
                    if v[i-1] != 0 {
                        let pct_change = (v[i] as f64 - v[i-1] as f64) / v[i-1] as f64 * 100.0;
                        pct_changes.push(Some(pct_change));
                    } else {
                        pct_changes.push(None);
                    }
                }
            },
            TinyColumn::OptFloat(v) => {
                pct_changes.push(None); // First value is always None
                for i in 1..v.len() {
                    match (v[i-1], v[i]) {
                        (Some(prev), Some(curr)) => {
                            if prev != 0.0 {
                                let pct_change = (curr - prev) / prev * 100.0;
                                pct_changes.push(Some(pct_change));
                            } else {
                                pct_changes.push(None);
                            }
                        },
                        (Some(prev), None) => pct_changes.push(None),
                        (None, Some(_)) => pct_changes.push(None),
                        (None, None) => pct_changes.push(None),
                    }
                }
            },
            TinyColumn::OptInt(v) => {
                pct_changes.push(None); // First value is always None
                for i in 1..v.len() {
                    match (v[i-1], v[i]) {
                        (Some(prev), Some(curr)) => {
                            if prev != 0 {
                                let pct_change = (curr as f64 - prev as f64) / prev as f64 * 100.0;
                                pct_changes.push(Some(pct_change));
                            } else {
                                pct_changes.push(None);
                            }
                        },
                        _ => pct_changes.push(None),
                    }
                }
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Percentage change only supported on numeric columns"
            )),
        }

        let mut new_columns = frame.columns.clone();
        new_columns.insert(format!("{}_pct_change", column), TinyColumn::OptFloat(pct_changes));
        
        Ok(TinyFrame {
            columns: new_columns,
            length: frame.length,
            py_objects: frame.py_objects.clone(),
        })
    }

    /// Calculate ranks for non-optional values
    fn calculate_ranks(
        sorted_values: &[(usize, f64)], 
        method: &RankMethod, 
        total_len: usize
    ) -> Vec<Option<f64>> {
        let mut result_ranks = vec![None; total_len];
        
        match method {
            RankMethod::Average => {
                let mut i = 0;
                while i < sorted_values.len() {
                    let current_value = sorted_values[i].1;
                    let mut j = i;
                    while j < sorted_values.len() && sorted_values[j].1 == current_value {
                        j += 1;
                    }
                    
                    // Calculate average rank for tied values
                    let avg_rank = (i + j + 1) as f64 / 2.0;
                    for k in i..j {
                        let original_idx = sorted_values[k].0;
                        if original_idx < result_ranks.len() {
                            result_ranks[original_idx] = Some(avg_rank);
                        }
                    }
                    i = j;
                }
            },
            RankMethod::Min => {
                let mut i = 0;
                while i < sorted_values.len() {
                    let current_value = sorted_values[i].1;
                    let mut j = i;
                    while j < sorted_values.len() && sorted_values[j].1 == current_value {
                        j += 1;
                    }
                    
                    // Use minimum rank for tied values
                    let min_rank = (i + 1) as f64;
                    for k in i..j {
                        let original_idx = sorted_values[k].0;
                        if original_idx < result_ranks.len() {
                            result_ranks[original_idx] = Some(min_rank);
                        }
                    }
                    i = j;
                }
            },
            RankMethod::Max => {
                let mut i = 0;
                while i < sorted_values.len() {
                    let current_value = sorted_values[i].1;
                    let mut j = i;
                    while j < sorted_values.len() && sorted_values[j].1 == current_value {
                        j += 1;
                    }
                    
                    // Use maximum rank for tied values
                    let max_rank = j as f64;
                    for k in i..j {
                        let original_idx = sorted_values[k].0;
                        if original_idx < result_ranks.len() {
                            result_ranks[original_idx] = Some(max_rank);
                        }
                    }
                    i = j;
                }
            },
            RankMethod::First => {
                for (rank, (original_index, _)) in sorted_values.iter().enumerate() {
                    if *original_index < result_ranks.len() {
                        result_ranks[*original_index] = Some((rank + 1) as f64);
                    }
                }
            },
            RankMethod::Dense => {
                let mut dense_rank = 1.0;
                let mut i = 0;
                while i < sorted_values.len() {
                    let current_value = sorted_values[i].1;
                    let mut j = i;
                    while j < sorted_values.len() && sorted_values[j].1 == current_value {
                        j += 1;
                    }
                    
                    // Use dense ranking (no gaps)
                    for k in i..j {
                        let original_idx = sorted_values[k].0;
                        if original_idx < result_ranks.len() {
                            result_ranks[original_idx] = Some(dense_rank);
                        }
                    }
                    dense_rank += 1.0;
                    i = j;
                }
            },
        }
        
        result_ranks
    }

    /// Calculate ranks for optional values
    fn calculate_ranks_optional(
        sorted_values: &[(usize, Option<f64>)], 
        method: &RankMethod, 
        total_len: usize
    ) -> Vec<Option<f64>> {
        let mut result_ranks = vec![None; total_len];
        
        // Separate null and non-null values
        let non_null_values: Vec<(usize, f64)> = sorted_values.iter()
            .filter_map(|(idx, val)| val.map(|v| (*idx, v)))
            .collect();
        
        // Calculate ranks for non-null values only
        let non_null_ranks = Self::calculate_ranks(&non_null_values, method, total_len);
        
        // Assign ranks to non-null values
        for (i, (original_idx, _)) in non_null_values.iter().enumerate() {
            if i < non_null_ranks.len() && *original_idx < result_ranks.len() {
                result_ranks[*original_idx] = non_null_ranks[*original_idx];
            }
        }
        
        // Null values remain None (already initialized)
        result_ranks
    }
}
