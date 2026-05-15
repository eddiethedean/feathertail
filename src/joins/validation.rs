//! Column-name validation for join operators (SRP: keep policy out of join assembly).

use crate::frame::TinyFrame;
use pyo3::prelude::*;
use std::collections::HashSet;

/// Enforce distinct output identifiers: keyed joins build a [`std::collections::HashMap`] of columns.
pub fn keyed_join_output_columns(
    left: &TinyFrame,
    right: &TinyFrame,
    left_on: &[String],
    right_on: &[String],
) -> PyResult<()> {
    let left_on_s: HashSet<&str> = left_on.iter().map(|s| s.as_str()).collect();
    let right_on_s: HashSet<&str> = right_on.iter().map(|s| s.as_str()).collect();
    let left_non_join: HashSet<&str> = left
        .columns
        .keys()
        .map(|s| s.as_str())
        .filter(|name| !left_on_s.contains(name))
        .collect();
    let right_non_join: HashSet<&str> = right
        .columns
        .keys()
        .map(|s| s.as_str())
        .filter(|name| !right_on_s.contains(name))
        .collect();

    let mut conflicts: Vec<&str> = left_non_join
        .intersection(&right_non_join)
        .copied()
        .collect();

    conflicts.extend(left_on_s.intersection(&right_non_join).copied());
    conflicts.extend(right_on_s.intersection(&left_non_join).copied());

    conflicts.sort_unstable();
    conflicts.dedup();
    if !conflicts.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "join: duplicate output column basename(s): [{}]; rename on one side (left_on='{}', right_on='{}')",
            conflicts.join(", "),
            left_on.join(", "),
            right_on.join(", ")
        )));
    }
    Ok(())
}

pub fn cross_join_columns_disjoint(left: &TinyFrame, right: &TinyFrame) -> PyResult<()> {
    let left_names: HashSet<&str> = left.columns.keys().map(|k| k.as_str()).collect();
    for rk in right.columns.keys() {
        if left_names.contains(rk.as_str()) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "cross_join: duplicate column name '{}'; rename one side before joining",
                rk
            )));
        }
    }
    Ok(())
}
