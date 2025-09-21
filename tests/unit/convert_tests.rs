#[cfg(test)]
mod tests {
    use feathertail::frame::{TinyFrame, TinyColumn, ValueEnum};
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    fn create_test_records() -> Vec<PyObject> {
        Python::with_gil(|py| {
            let records = vec![
                PyDict::new(py),
                PyDict::new(py),
                PyDict::new(py),
            ];
            
            // Add test data to first record
            records[0].set_item("age", 25).unwrap();
            records[0].set_item("name", "Alice").unwrap();
            records[0].set_item("score", 95.5).unwrap();
            
            // Add test data to second record
            records[1].set_item("age", 30).unwrap();
            records[1].set_item("name", "Bob").unwrap();
            records[1].set_item("score", 87.0).unwrap();
            
            // Add test data to third record
            records[2].set_item("age", 35).unwrap();
            records[2].set_item("name", "Charlie").unwrap();
            records[2].set_item("score", 92.5).unwrap();
            
            records.into_iter().map(|dict| dict.into()).collect()
        })
    }

    fn create_mixed_type_records() -> Vec<PyObject> {
        Python::with_gil(|py| {
            let records = vec![
                PyDict::new(py),
                PyDict::new(py),
                PyDict::new(py),
            ];
            
            // Mixed types in first record
            records[0].set_item("value", 42).unwrap();
            records[0].set_item("text", "hello").unwrap();
            
            // Mixed types in second record
            records[1].set_item("value", "world").unwrap();
            records[1].set_item("text", 3.14).unwrap();
            
            // Mixed types in third record
            records[2].set_item("value", true).unwrap();
            records[2].set_item("text", "mixed").unwrap();
            
            records.into_iter().map(|dict| dict.into()).collect()
        })
    }

    fn create_records_with_nulls() -> Vec<PyObject> {
        Python::with_gil(|py| {
            let records = vec![
                PyDict::new(py),
                PyDict::new(py),
                PyDict::new(py),
            ];
            
            // First record - no nulls
            records[0].set_item("age", 25).unwrap();
            records[0].set_item("name", "Alice").unwrap();
            
            // Second record - one null
            records[1].set_item("age", py.None()).unwrap();
            records[1].set_item("name", "Bob").unwrap();
            
            // Third record - no nulls
            records[2].set_item("age", 35).unwrap();
            records[2].set_item("name", "Charlie").unwrap();
            
            records.into_iter().map(|dict| dict.into()).collect()
        })
    }

    #[test]
    fn test_type_inference_single_type() {
        Python::with_gil(|py| {
            let records = create_test_records();
            let frame = TinyFrame::from_dicts(py, &records).unwrap();
            
            // Check that age column is inferred as Int
            if let Some(TinyColumn::Int(age_col)) = frame.columns.get("age") {
                assert_eq!(age_col.len(), 3);
                assert_eq!(age_col[0], 25);
                assert_eq!(age_col[1], 30);
                assert_eq!(age_col[2], 35);
            } else {
                panic!("Age column should be inferred as Int");
            }
            
            // Check that name column is inferred as Str
            if let Some(TinyColumn::Str(name_col)) = frame.columns.get("name") {
                assert_eq!(name_col.len(), 3);
                assert_eq!(name_col[0], "Alice");
                assert_eq!(name_col[1], "Bob");
                assert_eq!(name_col[2], "Charlie");
            } else {
                panic!("Name column should be inferred as Str");
            }
            
            // Check that score column is inferred as Float
            if let Some(TinyColumn::Float(score_col)) = frame.columns.get("score") {
                assert_eq!(score_col.len(), 3);
                assert!((score_col[0] - 95.5).abs() < 1e-10);
                assert!((score_col[1] - 87.0).abs() < 1e-10);
                assert!((score_col[2] - 92.5).abs() < 1e-10);
            } else {
                panic!("Score column should be inferred as Float");
            }
        });
    }

    #[test]
    fn test_type_inference_mixed_types() {
        Python::with_gil(|py| {
            let records = create_mixed_type_records();
            let frame = TinyFrame::from_dicts(py, &records).unwrap();
            
            // Check that value column is inferred as Mixed
            if let Some(TinyColumn::Mixed(value_col)) = frame.columns.get("value") {
                assert_eq!(value_col.len(), 3);
                assert!(matches!(value_col[0], ValueEnum::Int(42)));
                assert!(matches!(value_col[1], ValueEnum::Str(_)));
                assert!(matches!(value_col[2], ValueEnum::Bool(true)));
            } else {
                panic!("Value column should be inferred as Mixed");
            }
        });
    }

    #[test]
    fn test_type_inference_with_nulls() {
        Python::with_gil(|py| {
            let records = create_records_with_nulls();
            let frame = TinyFrame::from_dicts(py, &records).unwrap();
            
            // Check that age column is inferred as OptInt
            if let Some(TinyColumn::OptInt(age_col)) = frame.columns.get("age") {
                assert_eq!(age_col.len(), 3);
                assert_eq!(age_col[0], Some(25));
                assert_eq!(age_col[1], None);
                assert_eq!(age_col[2], Some(35));
            } else {
                panic!("Age column should be inferred as OptInt");
            }
            
            // Check that name column is inferred as Str (no nulls)
            if let Some(TinyColumn::Str(name_col)) = frame.columns.get("name") {
                assert_eq!(name_col.len(), 3);
                assert_eq!(name_col[0], "Alice");
                assert_eq!(name_col[1], "Bob");
                assert_eq!(name_col[2], "Charlie");
            } else {
                panic!("Name column should be inferred as Str");
            }
        });
    }

    #[test]
    fn test_empty_records_error() {
        Python::with_gil(|py| {
            let empty_records: Vec<PyObject> = vec![];
            let result = TinyFrame::from_dicts(py, &empty_records);
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_roundtrip_conversion() {
        Python::with_gil(|py| {
            let original_records = create_test_records();
            let frame = TinyFrame::from_dicts(py, &original_records).unwrap();
            let converted_records = frame.to_dicts(py).unwrap();
            
            // Basic check that we get the same number of records
            assert_eq!(original_records.len(), converted_records.len());
            assert_eq!(frame.len(), converted_records.len());
        });
    }
}
