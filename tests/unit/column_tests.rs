#[cfg(test)]
mod tests {
    use feathertail::frame::{TinyColumn, ValueEnum};

    #[test]
    fn test_int_column_creation() {
        let col = TinyColumn::Int(vec![1, 2, 3]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_opt_int_column_creation() {
        let col = TinyColumn::OptInt(vec![Some(1), None, Some(3)]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_float_column_creation() {
        let col = TinyColumn::Float(vec![1.0, 2.5, 3.14]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_str_column_creation() {
        let col = TinyColumn::Str(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_bool_column_creation() {
        let col = TinyColumn::Bool(vec![true, false, true]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_mixed_column_creation() {
        let col = TinyColumn::Mixed(vec![
            ValueEnum::Int(42),
            ValueEnum::Str("hello".to_string()),
            ValueEnum::Float(3.14),
        ]);
        assert_eq!(col.len(), 3);
    }

    #[test]
    fn test_opt_mixed_column_creation() {
        let col = TinyColumn::OptMixed(vec![
            Some(ValueEnum::Int(42)),
            None,
            Some(ValueEnum::Str("hello".to_string())),
        ]);
        assert_eq!(col.len(), 3);
    }

    // Test column length consistency
    #[test]
    fn test_column_length_consistency() {
        let test_cases = vec![
            TinyColumn::Int(vec![1, 2, 3, 4, 5]),
            TinyColumn::Float(vec![1.0, 2.0, 3.0]),
            TinyColumn::Str(vec!["a".to_string(), "b".to_string()]),
            TinyColumn::Bool(vec![true, false]),
            TinyColumn::OptInt(vec![Some(1), None, Some(3)]),
            TinyColumn::OptFloat(vec![Some(1.0), None]),
            TinyColumn::OptStr(vec![Some("a".to_string()), None, Some("c".to_string())]),
            TinyColumn::OptBool(vec![Some(true), None, Some(false)]),
        ];

        for col in test_cases {
            assert!(col.len() > 0, "Column should have positive length");
        }
    }

    // Test ValueEnum variants
    #[test]
    fn test_value_enum_variants() {
        let int_val = ValueEnum::Int(42);
        let float_val = ValueEnum::Float(3.14);
        let str_val = ValueEnum::Str("hello".to_string());
        let bool_val = ValueEnum::Bool(true);

        // Test that we can create all variants
        assert!(matches!(int_val, ValueEnum::Int(_)));
        assert!(matches!(float_val, ValueEnum::Float(_)));
        assert!(matches!(str_val, ValueEnum::Str(_)));
        assert!(matches!(bool_val, ValueEnum::Bool(_)));
    }
}
