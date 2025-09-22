#[cfg(test)]
mod integration_tests {
    use crate::simd::{SimdOps, SimdStringOps, SimdCapabilities, SimdType, get_simd_capabilities, get_best_simd_type};
    use std::thread;

    // Test that SIMD operations work correctly across different data patterns
    #[test]
    fn test_cross_platform_correctness() {
        let test_cases_i64 = vec![
            // Edge cases
            vec![],
            vec![0],
            vec![1],
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![i64::MIN, i64::MAX],
            
            // Small vectors
            vec![1, 2, 3, 4],
            
            // Medium vectors
            (0..100).collect::<Vec<i64>>(),
            
            // Large vectors
            (0..10000).collect::<Vec<i64>>(),
        ];

        let test_cases_f64 = vec![
            // Edge cases
            vec![],
            vec![0.0],
            vec![1.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0],
            vec![f64::MIN, f64::MAX],
            vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY],
            
            // Small vectors
            vec![1.0, 2.0, 3.0, 4.0],
            
            // Medium vectors
            (0..100).map(|i| i as f64).collect::<Vec<f64>>(),
            
            // Large vectors
            (0..10000).map(|i| i as f64).collect::<Vec<f64>>(),
        ];

        for (i, data_i64) in test_cases_i64.iter().enumerate() {
            if !data_i64.is_empty() {
                // Test i64 operations
                let sum = SimdOps::sum_i64(data_i64);
                let expected_sum: i64 = data_i64.iter().sum();
                assert_eq!(sum, expected_sum, "sum_i64 failed for test case {}", i);
                
                if !data_i64.is_empty() {
                    let (min, max) = SimdOps::min_max_i64(data_i64);
                    let expected_min = *data_i64.iter().min().unwrap();
                    let expected_max = *data_i64.iter().max().unwrap();
                    assert_eq!(min, expected_min, "min_i64 failed for test case {}", i);
                    assert_eq!(max, expected_max, "max_i64 failed for test case {}", i);
                }
            }
        }

        for (i, data_f64) in test_cases_f64.iter().enumerate() {
            if !data_f64.is_empty() {
                // Test f64 operations
                let sum = SimdOps::sum_f64(data_f64);
                let expected_sum: f64 = data_f64.iter().sum();
                assert!((sum - expected_sum).abs() < 1e-10, 
                    "sum_f64 failed for test case {}: expected {}, actual {}", i, expected_sum, sum);
                
                if !data_f64.is_empty() {
                    let (min, max) = SimdOps::min_max_f64(data_f64);
                    let expected_min = *data_f64.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    let expected_max = *data_f64.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    assert!((min - expected_min).abs() < 1e-10, 
                        "min_f64 failed for test case {}: expected {}, actual {}", i, expected_min, min);
                    assert!((max - expected_max).abs() < 1e-10, 
                        "max_f64 failed for test case {}: expected {}, actual {}", i, expected_max, max);
                }
            }
        }
    }

    // Test that SIMD operations are consistent across multiple calls
    #[test]
    fn test_deterministic_behavior() {
        let data = (0..1000).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
        
        // Run the same operation multiple times and ensure consistent results
        let results: Vec<f64> = (0..100).map(|_| SimdOps::sum_f64(&data)).collect();
        
        // All results should be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i], "SIMD operations should be deterministic");
        }
    }

    // Test that SIMD operations handle different memory alignments
    #[test]
    fn test_memory_alignment_handling() {
        let base_data = (0..1000).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
        
        // Test with different starting offsets to test alignment handling
        for offset in 0..8 {
            if offset < base_data.len() {
                let data = &base_data[offset..];
                let sum = SimdOps::sum_f64(data);
                let expected_sum: f64 = data.iter().sum();
                assert!((sum - expected_sum).abs() < 1e-10, 
                    "Memory alignment test failed for offset {}: expected {}, actual {}", 
                    offset, expected_sum, sum);
            }
        }
    }

    // Test that SIMD operations work with different data patterns
    #[test]
    fn test_data_pattern_handling() {
        let patterns = vec![
            // Ascending
            (0..1000).map(|i| i as f64).collect::<Vec<f64>>(),
            // Descending
            (0..1000).rev().map(|i| i as f64).collect::<Vec<f64>>(),
            // Alternating
            (0..1000).map(|i| if i % 2 == 0 { i as f64 } else { -(i as f64) }).collect::<Vec<f64>>(),
            // Random-like
            (0..1000).map(|i| ((i * 7) % 1000) as f64).collect::<Vec<f64>>(),
            // Constant
            vec![42.0; 1000],
            // Sparse
            (0..1000).map(|i| if i % 10 == 0 { i as f64 } else { 0.0 }).collect::<Vec<f64>>(),
        ];

        for (i, data) in patterns.iter().enumerate() {
            let sum = SimdOps::sum_f64(data);
            let expected_sum: f64 = data.iter().sum();
            assert!((sum - expected_sum).abs() < 1e-10, 
                "Data pattern test failed for pattern {}: expected {}, actual {}", 
                i, expected_sum, sum);
            
            let mean = SimdOps::mean_f64(data);
            let expected_mean = expected_sum / data.len() as f64;
            assert!((mean - expected_mean).abs() < 1e-10, 
                "Mean test failed for pattern {}: expected {}, actual {}", 
                i, expected_mean, mean);
        }
    }

    // Test that SIMD operations work correctly with string data
    #[test]
    fn test_string_operation_correctness() {
        let test_strings = vec![
            "",
            "a",
            "ab",
            "abc",
            "hello",
            "HELLO",
            "Hello",
            "hElLo",
            "hello world",
            "HELLO WORLD",
            "Hello World",
            "hElLo WoRlD",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "1234567890",
            "!@#$%^&*()",
            "a very long string that should test the SIMD string operations thoroughly",
        ];

        for string in test_strings {
            // Test uppercase
            let upper = SimdStringOps::to_uppercase_simd(string);
            let expected_upper = string.to_uppercase();
            assert_eq!(upper, expected_upper, 
                "to_uppercase_simd failed for string: '{}'", string);
            
            // Test lowercase
            let lower = SimdStringOps::to_lowercase_simd(string);
            let expected_lower = string.to_lowercase();
            assert_eq!(lower, expected_lower, 
                "to_lowercase_simd failed for string: '{}'", string);
            
            // Test contains
            if !string.is_empty() {
                let contains_result = SimdStringOps::contains_simd(string, &string[0..1]);
                let expected_contains = string.contains(&string[0..1]);
                assert_eq!(contains_result, expected_contains, 
                    "contains_simd failed for string: '{}', needle: '{}'", 
                    string, &string[0..1]);
            }
        }
    }

    // Test that SIMD operations handle concurrent access correctly
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        
        let data = (0..10000).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
        let num_threads = 4;
        let iterations_per_thread = 100;
        
        let handles: Vec<_> = (0..num_threads).map(|_| {
            let data = data.clone();
            thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    let sum = SimdOps::sum_f64(&data);
                    let expected_sum: f64 = data.iter().sum();
                    assert!((sum - expected_sum).abs() < 1e-10, 
                        "Concurrent access test failed: expected {}, actual {}", 
                        expected_sum, sum);
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Test that SIMD operations work correctly with different vector sizes
    #[test]
    fn test_vector_size_handling() {
        let sizes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129];
        
        for size in sizes {
            let data = (0..size).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
            
            let sum = SimdOps::sum_f64(&data);
            let expected_sum: f64 = data.iter().sum();
            assert!((sum - expected_sum).abs() < 1e-10, 
                "Vector size test failed for size {}: expected {}, actual {}", 
                size, expected_sum, sum);
            
            if !data.is_empty() {
                let mean = SimdOps::mean_f64(&data);
                let expected_mean = expected_sum / data.len() as f64;
                assert!((mean - expected_mean).abs() < 1e-10, 
                    "Mean test failed for size {}: expected {}, actual {}", 
                    size, expected_mean, mean);
            }
        }
    }

    // Test that SIMD operations handle NaN and Infinity correctly
    #[test]
    fn test_special_values_handling() {
        let special_values = vec![
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            0.0,
            -0.0,
        ];
        
        for value in special_values {
            let data = vec![value; 100];
            
            // Test sum (should handle NaN correctly)
            let sum = SimdOps::sum_f64(&data);
            if value.is_nan() {
                assert!(sum.is_nan(), "Sum should be NaN when input contains NaN");
            } else {
                let expected_sum = value * 100.0;
                assert!((sum - expected_sum).abs() < 1e-10 || 
                       (sum.is_infinite() && expected_sum.is_infinite() && sum.signum() == expected_sum.signum()), 
                    "Sum test failed for special value {}: expected {}, actual {}", 
                    value, expected_sum, sum);
            }
        }
    }

    // Test that SIMD operations are thread-safe
    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;
        
        let data = Arc::new((0..1000).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>());
        let num_threads = 8;
        let iterations_per_thread = 50;
        
        let handles: Vec<_> = (0..num_threads).map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    // Test various operations
                    let _ = SimdOps::sum_f64(&data);
                    let _ = SimdOps::mean_f64(&data);
                    let _ = SimdOps::min_max_f64(&data);
                    let _ = SimdOps::variance_f64(&data);
                    let _ = SimdOps::std_dev_f64(&data);
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Test that SIMD operations work correctly with different data types
    #[test]
    fn test_data_type_handling() {
        // Test i64 operations
        let data_i64 = vec![i64::MIN, -1, 0, 1, i64::MAX];
        let sum_i64 = SimdOps::sum_i64(&data_i64);
        let expected_sum_i64: i64 = data_i64.iter().sum();
        assert_eq!(sum_i64, expected_sum_i64, "i64 sum test failed");
        
        // Test f64 operations
        let data_f64 = vec![f64::MIN, -1.0, 0.0, 1.0, f64::MAX];
        let sum_f64 = SimdOps::sum_f64(&data_f64);
        let expected_sum_f64: f64 = data_f64.iter().sum();
        assert!((sum_f64 - expected_sum_f64).abs() < 1e-10, 
            "f64 sum test failed: expected {}, actual {}", expected_sum_f64, sum_f64);
    }

    // Test that SIMD operations handle empty vectors correctly
    #[test]
    fn test_empty_vector_handling() {
        let empty_i64: Vec<i64> = vec![];
        let empty_f64: Vec<f64> = vec![];
        
        // Test i64 operations
        assert_eq!(SimdOps::sum_i64(&empty_i64), 0);
        assert_eq!(SimdOps::min_max_i64(&empty_i64), (0, 0));
        
        // Test f64 operations
        assert_eq!(SimdOps::sum_f64(&empty_f64), 0.0);
        assert_eq!(SimdOps::mean_f64(&empty_f64), 0.0);
        assert_eq!(SimdOps::variance_f64(&empty_f64), 0.0);
        assert_eq!(SimdOps::std_dev_f64(&empty_f64), 0.0);
        assert_eq!(SimdOps::min_max_f64(&empty_f64), (0.0, 0.0));
    }

    // Test that SIMD operations handle single element vectors correctly
    #[test]
    fn test_single_element_handling() {
        let single_i64 = vec![42];
        let single_f64 = vec![3.14];
        
        // Test i64 operations
        assert_eq!(SimdOps::sum_i64(&single_i64), 42);
        assert_eq!(SimdOps::min_max_i64(&single_i64), (42, 42));
        
        // Test f64 operations
        assert_eq!(SimdOps::sum_f64(&single_f64), 3.14);
        assert_eq!(SimdOps::mean_f64(&single_f64), 3.14);
        assert_eq!(SimdOps::variance_f64(&single_f64), 0.0);
        assert_eq!(SimdOps::std_dev_f64(&single_f64), 0.0);
        assert_eq!(SimdOps::min_max_f64(&single_f64), (3.14, 3.14));
    }
}
