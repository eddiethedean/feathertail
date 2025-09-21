use pyo3::prelude::*;
use crate::frame::{TinyFrame, TinyColumn};
use std::collections::HashMap;

/// Calculate correlation matrix for numeric columns
pub fn corr_impl(frame: &TinyFrame) -> PyResult<TinyFrame> {
    let numeric_columns = get_numeric_columns(frame)?;
    let mut corr_matrix: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for (col1_name, col1_data) in &numeric_columns {
        let mut col1_corr: HashMap<String, f64> = HashMap::new();
        
        for (col2_name, col2_data) in &numeric_columns {
            let correlation = calculate_correlation(col1_data, col2_data)?;
            col1_corr.insert(col2_name.clone(), correlation);
        }
        
        corr_matrix.insert(col1_name.clone(), col1_corr);
    }

    build_correlation_frame(corr_matrix, &numeric_columns)
}

/// Calculate covariance matrix for numeric columns
pub fn cov_impl(frame: &TinyFrame) -> PyResult<TinyFrame> {
    let numeric_columns = get_numeric_columns(frame)?;
    let mut cov_matrix: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for (col1_name, col1_data) in &numeric_columns {
        let mut col1_cov: HashMap<String, f64> = HashMap::new();
        
        for (col2_name, col2_data) in &numeric_columns {
            let covariance = calculate_covariance(col1_data, col2_data)?;
            col1_cov.insert(col2_name.clone(), covariance);
        }
        
        cov_matrix.insert(col1_name.clone(), col1_cov);
    }

    build_correlation_frame(cov_matrix, &numeric_columns)
}

/// Calculate correlation between two specific columns
pub fn corr_with_impl(frame: &TinyFrame, column1: &str, column2: &str) -> PyResult<f64> {
    let col1 = frame.columns.get(column1)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column1)
        ))?;
    
    let col2 = frame.columns.get(column2)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column2)
        ))?;

    calculate_correlation(col1, col2)
}

/// Calculate covariance between two specific columns
pub fn cov_with_impl(frame: &TinyFrame, column1: &str, column2: &str) -> PyResult<f64> {
    let col1 = frame.columns.get(column1)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column1)
        ))?;
    
    let col2 = frame.columns.get(column2)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            format!("Column '{}' not found", column2)
        ))?;

    calculate_covariance(col1, col2)
}

fn get_numeric_columns(frame: &TinyFrame) -> PyResult<HashMap<String, &TinyColumn>> {
    let mut numeric_columns = HashMap::new();
    
    for (col_name, col_data) in &frame.columns {
        match col_data {
            TinyColumn::Int(_) | TinyColumn::Float(_) | 
            TinyColumn::OptInt(_) | TinyColumn::OptFloat(_) => {
                numeric_columns.insert(col_name.clone(), col_data);
            }
            _ => {} // Skip non-numeric columns
        }
    }
    
    Ok(numeric_columns)
}

/// Extract numeric values from two columns for correlation calculations
fn extract_numeric_values(col1: &TinyColumn, col2: &TinyColumn) -> PyResult<(Vec<f64>, Vec<f64>)> {
    let values1 = match col1 {
        TinyColumn::Int(v) => v.iter().map(|&x| x as f64).collect(),
        TinyColumn::Float(v) => v.clone(),
        TinyColumn::OptInt(v) => v.iter().filter_map(|&x| x.map(|v| v as f64)).collect(),
        TinyColumn::OptFloat(v) => v.iter().filter_map(|&x| x).collect(),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Correlation only supported on numeric columns"
        )),
    };

    let values2 = match col2 {
        TinyColumn::Int(v) => v.iter().map(|&x| x as f64).collect(),
        TinyColumn::Float(v) => v.clone(),
        TinyColumn::OptInt(v) => v.iter().filter_map(|&x| x.map(|v| v as f64)).collect(),
        TinyColumn::OptFloat(v) => v.iter().filter_map(|&x| x).collect(),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Correlation only supported on numeric columns"
        )),
    };

    Ok((values1, values2))
}

fn calculate_correlation(col1: &TinyColumn, col2: &TinyColumn) -> PyResult<f64> {
    let (values1, values2) = extract_numeric_values(col1, col2)?;
    
    if values1.len() != values2.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Columns must have the same length"
        ));
    }

    if values1.is_empty() {
        return Ok(0.0);
    }

    let n = values1.len() as f64;
    let mean1 = values1.iter().sum::<f64>() / n;
    let mean2 = values2.iter().sum::<f64>() / n;

    let numerator: f64 = values1.iter()
        .zip(values2.iter())
        .map(|(x, y)| (x - mean1) * (y - mean2))
        .sum();

    let sum_sq1: f64 = values1.iter()
        .map(|x| (x - mean1).powi(2))
        .sum();
    let sum_sq2: f64 = values2.iter()
        .map(|y| (y - mean2).powi(2))
        .sum();

    let denominator = (sum_sq1 * sum_sq2).sqrt();
    
    if denominator == 0.0 {
        Ok(0.0)
    } else {
        Ok(numerator / denominator)
    }
}

fn calculate_covariance(col1: &TinyColumn, col2: &TinyColumn) -> PyResult<f64> {
    let (values1, values2) = extract_numeric_values(col1, col2)?;
    
    if values1.len() != values2.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Columns must have the same length"
        ));
    }

    if values1.is_empty() {
        return Ok(0.0);
    }

    let n = values1.len() as f64;
    let mean1 = values1.iter().sum::<f64>() / n;
    let mean2 = values2.iter().sum::<f64>() / n;

    let covariance: f64 = values1.iter()
        .zip(values2.iter())
        .map(|(x, y)| (x - mean1) * (y - mean2))
        .sum();

    Ok(covariance / n)
}

fn build_correlation_frame(matrix: HashMap<String, HashMap<String, f64>>, 
                          numeric_columns: &HashMap<String, &TinyColumn>) -> PyResult<TinyFrame> {
    let mut columns: HashMap<String, TinyColumn> = HashMap::new();
    let column_names: Vec<String> = numeric_columns.keys().cloned().collect();
    
    // Add column names as the first column
    columns.insert("column".to_string(), TinyColumn::Str(column_names.clone()));

    // Add correlation values for each column
    for col_name in &column_names {
        let mut values = Vec::new();
        for other_col in &column_names {
            if let Some(corr_map) = matrix.get(col_name) {
                if let Some(&corr_value) = corr_map.get(other_col) {
                    values.push(corr_value);
                } else {
                    values.push(0.0);
                }
            } else {
                values.push(0.0);
            }
        }
        columns.insert(col_name.clone(), TinyColumn::Float(values));
    }

    Ok(TinyFrame {
        columns,
        length: column_names.len(),
        py_objects: HashMap::new(),
    })
}