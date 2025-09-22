#[cfg(test)]
mod tests {
    use crate::simd::{SimdOps, SimdStringOps, SimdCapabilities, SimdType, get_simd_capabilities, get_best_simd_type};
    use std::time::Instant;

    // Test data generators
    fn generate_test_data_i64(size: usize) -> Vec<i64> {
        (0..size).map(|i| (i as i64) * 3 - 1000).collect()
    }

    fn generate_test_data_f64(size: usize) -> Vec<f64> {
        (0..size).map(|i| (i as f64) * 0.5 + 100.0).collect()
    }

    fn generate_test_strings() -> Vec<String> {
        vec![
            "hello world".to_string(),
            "HELLO WORLD".to_string(),
            "Hello World".to_string(),
            "hElLo WoRlD".to_string(),
            "".to_string(),
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abcdefghijklmnopqrstuvwxyz".to_string(),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        ]
    }

    // Test SIMD operations correctness
    #[test]
    fn test_simd_sum_i64() {
        let test_cases = vec![
            vec![],
            vec![1],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            generate_test_data_i64(100),
            generate_test_data_i64(1000),
        ];

        for data in test_cases {
            let expected: i64 = data.iter().sum();
            let actual = SimdOps::sum_i64(&data);
            assert_eq!(actual, expected, "sum_i64 failed for data: {:?}", data);
        }
    }

    #[test]
    fn test_simd_sum_f64() {
        let test_cases = vec![
            vec![],
            vec![1.0],
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            generate_test_data_f64(100),
            generate_test_data_f64(1000),
        ];

        for data in test_cases {
            let expected: f64 = data.iter().sum();
            let actual = SimdOps::sum_f64(&data);
            assert!((actual - expected).abs() < 1e-10, 
                "sum_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected, actual);
        }
    }

    #[test]
    fn test_simd_mean_f64() {
        let test_cases = vec![
            vec![],
            vec![1.0],
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            generate_test_data_f64(100),
            generate_test_data_f64(1000),
        ];

        for data in test_cases {
            if data.is_empty() {
                continue; // Skip empty case for mean
            }
            let expected: f64 = data.iter().sum::<f64>() / data.len() as f64;
            let actual = SimdOps::mean_f64(&data);
            assert!((actual - expected).abs() < 1e-10, 
                "mean_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected, actual);
        }
    }

    #[test]
    fn test_simd_min_max_i64() {
        let test_cases = vec![
            vec![],
            vec![1],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![-1, -2, -3, -4],
            vec![0, 0, 0, 0],
            generate_test_data_i64(100),
            generate_test_data_i64(1000),
        ];

        for data in test_cases {
            if data.is_empty() {
                continue; // Skip empty case
            }
            let expected_min = *data.iter().min().unwrap();
            let expected_max = *data.iter().max().unwrap();
            let (actual_min, actual_max) = SimdOps::min_max_i64(&data);
            assert_eq!(actual_min, expected_min, "min_i64 failed for data: {:?}", data);
            assert_eq!(actual_max, expected_max, "max_i64 failed for data: {:?}", data);
        }
    }

    #[test]
    fn test_simd_min_max_f64() {
        let test_cases = vec![
            vec![],
            vec![1.0],
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![-1.0, -2.0, -3.0, -4.0],
            vec![0.0, 0.0, 0.0, 0.0],
            generate_test_data_f64(100),
            generate_test_data_f64(1000),
        ];

        for data in test_cases {
            if data.is_empty() {
                continue; // Skip empty case
            }
            let expected_min = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let expected_max = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let (actual_min, actual_max) = SimdOps::min_max_f64(&data);
            assert!((actual_min - expected_min).abs() < 1e-10, 
                "min_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected_min, actual_min);
            assert!((actual_max - expected_max).abs() < 1e-10, 
                "max_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected_max, actual_max);
        }
    }

    #[test]
    fn test_simd_variance_f64() {
        let test_cases = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            generate_test_data_f64(100),
            generate_test_data_f64(1000),
        ];

        for data in test_cases {
            if data.len() < 2 {
                continue; // Skip cases with insufficient data
            }
            let mean = data.iter().sum::<f64>() / data.len() as f64;
            let expected = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
            let actual = SimdOps::variance_f64(&data);
            assert!((actual - expected).abs() < 1e-10, 
                "variance_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected, actual);
        }
    }

    #[test]
    fn test_simd_std_dev_f64() {
        let test_cases = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
            generate_test_data_f64(100),
            generate_test_data_f64(1000),
        ];

        for data in test_cases {
            if data.len() < 2 {
                continue; // Skip cases with insufficient data
            }
            let variance = SimdOps::variance_f64(&data);
            let expected = variance.sqrt();
            let actual = SimdOps::std_dev_f64(&data);
            assert!((actual - expected).abs() < 1e-10, 
                "std_dev_f64 failed for data: {:?}, expected: {}, actual: {}", data, expected, actual);
        }
    }

    #[test]
    fn test_simd_dot_product_f64() {
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
            (generate_test_data_f64(100), generate_test_data_f64(100)),
            (generate_test_data_f64(1000), generate_test_data_f64(1000)),
        ];

        for (a, b) in test_cases {
            let expected: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let actual = SimdOps::dot_product_f64(&a, &b);
            assert!((actual - expected).abs() < 1e-10, 
                "dot_product_f64 failed for a: {:?}, b: {:?}, expected: {}, actual: {}", a, b, expected, actual);
        }
    }

    #[test]
    fn test_simd_add_f64() {
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
            (generate_test_data_f64(100), generate_test_data_f64(100)),
            (generate_test_data_f64(1000), generate_test_data_f64(1000)),
        ];

        for (a, b) in test_cases {
            let expected: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
            let actual = SimdOps::add_f64(&a, &b);
            assert_eq!(actual.len(), expected.len(), "add_f64 length mismatch");
            for (i, (exp, act)) in expected.iter().zip(actual.iter()).enumerate() {
                assert!((act - exp).abs() < 1e-10, 
                    "add_f64 failed at index {}: expected {}, actual {}", i, exp, act);
            }
        }
    }

    #[test]
    fn test_simd_mul_f64() {
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
            (generate_test_data_f64(100), generate_test_data_f64(100)),
            (generate_test_data_f64(1000), generate_test_data_f64(1000)),
        ];

        for (a, b) in test_cases {
            let expected: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x * y).collect();
            let actual = SimdOps::mul_f64(&a, &b);
            assert_eq!(actual.len(), expected.len(), "mul_f64 length mismatch");
            for (i, (exp, act)) in expected.iter().zip(actual.iter()).enumerate() {
                assert!((act - exp).abs() < 1e-10, 
                    "mul_f64 failed at index {}: expected {}, actual {}", i, exp, act);
            }
        }
    }

    // Test string operations
    #[test]
    fn test_simd_string_uppercase() {
        let test_cases = generate_test_strings();

        for input in test_cases {
            let expected = input.to_uppercase();
            let actual = SimdStringOps::to_uppercase_simd(&input);
            assert_eq!(actual, expected, "to_uppercase_simd failed for input: '{}'", input);
        }
    }

    #[test]
    fn test_simd_string_lowercase() {
        let test_cases = generate_test_strings();

        for input in test_cases {
            let expected = input.to_lowercase();
            let actual = SimdStringOps::to_lowercase_simd(&input);
            assert_eq!(actual, expected, "to_lowercase_simd failed for input: '{}'", input);
        }
    }

    #[test]
    fn test_simd_string_contains() {
        let test_cases = vec![
            ("hello world", "world", true),
            ("hello world", "hello", true),
            ("hello world", "lo wo", true),
            ("hello world", "xyz", false),
            ("hello world", "", true),
            ("", "hello", false),
            ("", "", true),
            ("a", "a", true),
            ("a", "b", false),
            ("abcdefghijklmnopqrstuvwxyz", "mnop", true),
            ("abcdefghijklmnopqrstuvwxyz", "xyz", true),
            ("abcdefghijklmnopqrstuvwxyz", "abc", true),
        ];

        for (haystack, needle, expected) in test_cases {
            let actual = SimdStringOps::contains_simd(haystack, needle);
            assert_eq!(actual, expected, 
                "contains_simd failed for haystack: '{}', needle: '{}', expected: {}, actual: {}", 
                haystack, needle, expected, actual);
        }
    }

    // Test CPU feature detection
    #[test]
    fn test_cpu_feature_detection() {
        let capabilities = SimdCapabilities::detect();
        
        // Test that capabilities are detected correctly
        assert!(capabilities.avx2 || capabilities.neon || !capabilities.has_simd(), 
            "CPU feature detection should be consistent");
        
        // Test that we can get the best SIMD type
        let simd_type = capabilities.get_best_simd_type();
        match simd_type {
            SimdType::AVX2 => assert!(capabilities.avx2, "AVX2 should be detected if type is AVX2"),
            SimdType::NEON => assert!(capabilities.neon, "NEON should be detected if type is NEON"),
            SimdType::Scalar => assert!(!capabilities.has_simd(), "Scalar should be used when no SIMD available"),
        }
    }

    // Test SIMD type consistency
    #[test]
    fn test_simd_type_consistency() {
        let simd_type = get_best_simd_type();
        let capabilities = get_simd_capabilities();
        
        match simd_type {
            SimdType::AVX2 => assert!(capabilities.avx2, "AVX2 type should match capabilities"),
            SimdType::NEON => assert!(capabilities.neon, "NEON type should match capabilities"),
            SimdType::Scalar => assert!(!capabilities.has_simd(), "Scalar type should match capabilities"),
        }
    }

    // Performance benchmarks (basic)
    #[test]
    fn test_performance_benchmark() {
        let large_data = generate_test_data_f64(10000);
        let iterations = 1000;
        
        // Benchmark sum operation
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = SimdOps::sum_f64(&large_data);
        }
        let duration = start.elapsed();
        
        println!("SIMD sum_f64 performance: {} iterations in {:?} ({:?} per iteration)", 
                iterations, duration, duration / iterations);
        
        // Benchmark should complete in reasonable time
        assert!(duration.as_millis() < 1000, "Performance test took too long: {:?}", duration);
    }

    // Test edge cases
    #[test]
    fn test_edge_cases() {
        // Empty vectors
        assert_eq!(SimdOps::sum_i64(&[]), 0);
        assert_eq!(SimdOps::sum_f64(&[]), 0.0);
        assert_eq!(SimdOps::mean_f64(&[]), 0.0);
        assert_eq!(SimdOps::variance_f64(&[]), 0.0);
        assert_eq!(SimdOps::std_dev_f64(&[]), 0.0);
        
        // Single element
        assert_eq!(SimdOps::sum_i64(&[42]), 42);
        assert_eq!(SimdOps::sum_f64(&[3.14]), 3.14);
        assert_eq!(SimdOps::mean_f64(&[3.14]), 3.14);
        assert_eq!(SimdOps::variance_f64(&[3.14]), 0.0);
        assert_eq!(SimdOps::std_dev_f64(&[3.14]), 0.0);
        
        // Two elements
        assert_eq!(SimdOps::variance_f64(&[1.0, 2.0]), 0.5);
        assert!((SimdOps::std_dev_f64(&[1.0, 2.0]) - 0.7071067811865476).abs() < 1e-10);
    }

    // Test that SIMD operations are deterministic
    #[test]
    fn test_deterministic_behavior() {
        let data = generate_test_data_f64(1000);
        
        // Run the same operation multiple times
        let result1 = SimdOps::sum_f64(&data);
        let result2 = SimdOps::sum_f64(&data);
        let result3 = SimdOps::sum_f64(&data);
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}
