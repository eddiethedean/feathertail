#[cfg(test)]
mod property_tests {
    use crate::simd::{SimdOps, SimdStringOps, SimdCapabilities, SimdType, get_simd_capabilities, get_best_simd_type};

    // Property-based tests for SIMD operations
    // These tests verify mathematical properties that should hold for all inputs

    #[test]
    fn test_sum_commutativity() {
        // Sum should be commutative: sum(a + b) = sum(a) + sum(b)
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
            (vec![-1.0, -2.0, -3.0], vec![1.0, 2.0, 3.0]),
        ];

        for (a, b) in test_cases {
            let sum_a = SimdOps::sum_f64(&a);
            let sum_b = SimdOps::sum_f64(&b);
            let sum_combined = SimdOps::sum_f64(&[&a[..], &b[..]].concat());
            
            assert!((sum_combined - (sum_a + sum_b)).abs() < 1e-10, 
                "Sum commutativity failed for a: {:?}, b: {:?}", a, b);
        }
    }

    #[test]
    fn test_sum_associativity() {
        // Sum should be associative: sum(a) + sum(b) + sum(c) = sum(a + b + c)
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = vec![7.0, 8.0, 9.0];
        
        let sum_a = SimdOps::sum_f64(&a);
        let sum_b = SimdOps::sum_f64(&b);
        let sum_c = SimdOps::sum_f64(&c);
        let sum_combined = SimdOps::sum_f64(&[&a[..], &b[..], &c[..]].concat());
        
        assert!((sum_combined - (sum_a + sum_b + sum_c)).abs() < 1e-10, 
            "Sum associativity failed");
    }

    #[test]
    fn test_mean_property() {
        // Mean should satisfy: mean = sum / count
        let test_cases = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![-1.0, -2.0, -3.0, -4.0],
            (0..100).map(|i| i as f64).collect::<Vec<f64>>(),
        ];

        for data in test_cases {
            if !data.is_empty() {
                let sum = SimdOps::sum_f64(&data);
                let mean = SimdOps::mean_f64(&data);
                let expected_mean = sum / data.len() as f64;
                
                assert!((mean - expected_mean).abs() < 1e-10, 
                    "Mean property failed for data: {:?}", data);
            }
        }
    }

    #[test]
    fn test_min_max_properties() {
        // Min should be <= max, and both should be in the data
        let test_cases = vec![
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![1, 1, 1, 1, 1],
            vec![-1, -2, -3, -4, -5],
            vec![0, 0, 0, 0, 0],
            (0..1000).collect::<Vec<i64>>(),
        ];

        for data in test_cases {
            if !data.is_empty() {
                let (min, max) = SimdOps::min_max_i64(&data);
                
                // Min should be <= max
                assert!(min <= max, "Min should be <= max for data: {:?}", data);
                
                // Min and max should be in the data
                assert!(data.contains(&min), "Min should be in data: {:?}", data);
                assert!(data.contains(&max), "Max should be in data: {:?}", data);
                
                // All elements should be >= min and <= max
                for &value in &data {
                    assert!(value >= min, "All values should be >= min for data: {:?}", data);
                    assert!(value <= max, "All values should be <= max for data: {:?}", data);
                }
            }
        }
    }

    #[test]
    fn test_variance_properties() {
        // Variance should be >= 0
        let test_cases = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
            vec![-1.0, -2.0, -3.0, -4.0, -5.0],
            (0..100).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>(),
        ];

        for data in test_cases {
            if data.len() >= 2 {
                let variance = SimdOps::variance_f64(&data);
                assert!(variance >= 0.0, "Variance should be >= 0 for data: {:?}", data);
                
                // If all values are the same, variance should be 0
                if data.iter().all(|&x| (x - data[0]).abs() < 1e-10) {
                    assert!(variance.abs() < 1e-10, "Variance should be 0 for constant data: {:?}", data);
                }
            }
        }
    }

    #[test]
    fn test_std_dev_properties() {
        // Standard deviation should be >= 0 and std_dev^2 = variance
        let test_cases = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
            (0..100).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>(),
        ];

        for data in test_cases {
            if data.len() >= 2 {
                let variance = SimdOps::variance_f64(&data);
                let std_dev = SimdOps::std_dev_f64(&data);
                
                assert!(std_dev >= 0.0, "Standard deviation should be >= 0 for data: {:?}", data);
                assert!((std_dev * std_dev - variance).abs() < 1e-10, 
                    "std_dev^2 should equal variance for data: {:?}", data);
            }
        }
    }

    #[test]
    fn test_dot_product_properties() {
        // Dot product should be commutative: a·b = b·a
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
            (vec![-1.0, -2.0, -3.0], vec![1.0, 2.0, 3.0]),
        ];

        for (a, b) in test_cases {
            let dot_ab = SimdOps::dot_product_f64(&a, &b);
            let dot_ba = SimdOps::dot_product_f64(&b, &a);
            
            assert!((dot_ab - dot_ba).abs() < 1e-10, 
                "Dot product should be commutative for a: {:?}, b: {:?}", a, b);
        }
    }

    #[test]
    fn test_dot_product_distributivity() {
        // Dot product should be distributive: a·(b + c) = a·b + a·c
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = vec![7.0, 8.0, 9.0];
        
        let b_plus_c = SimdOps::add_f64(&b, &c);
        let dot_a_bc = SimdOps::dot_product_f64(&a, &b_plus_c);
        
        let dot_ab = SimdOps::dot_product_f64(&a, &b);
        let dot_ac = SimdOps::dot_product_f64(&a, &c);
        let dot_ab_plus_ac = dot_ab + dot_ac;
        
        assert!((dot_a_bc - dot_ab_plus_ac).abs() < 1e-10, 
            "Dot product should be distributive");
    }

    #[test]
    fn test_add_commutativity() {
        // Addition should be commutative: a + b = b + a
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
        ];

        for (a, b) in test_cases {
            let add_ab = SimdOps::add_f64(&a, &b);
            let add_ba = SimdOps::add_f64(&b, &a);
            
            assert_eq!(add_ab.len(), add_ba.len(), "Length should be equal");
            for (i, (x, y)) in add_ab.iter().zip(add_ba.iter()).enumerate() {
                assert!((x - y).abs() < 1e-10, 
                    "Addition should be commutative at index {}: a: {:?}, b: {:?}", i, a, b);
            }
        }
    }

    #[test]
    fn test_add_associativity() {
        // Addition should be associative: (a + b) + c = a + (b + c)
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = vec![7.0, 8.0, 9.0];
        
        let ab = SimdOps::add_f64(&a, &b);
        let abc = SimdOps::add_f64(&ab, &c);
        
        let bc = SimdOps::add_f64(&b, &c);
        let abc2 = SimdOps::add_f64(&a, &bc);
        
        assert_eq!(abc.len(), abc2.len(), "Length should be equal");
        for (i, (x, y)) in abc.iter().zip(abc2.iter()).enumerate() {
            assert!((x - y).abs() < 1e-10, 
                "Addition should be associative at index {}", i);
        }
    }

    #[test]
    fn test_mul_commutativity() {
        // Multiplication should be commutative: a * b = b * a
        let test_cases = vec![
            (vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]),
            (vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]),
            (vec![0.0, 0.0, 0.0], vec![1.0, 2.0, 3.0]),
        ];

        for (a, b) in test_cases {
            let mul_ab = SimdOps::mul_f64(&a, &b);
            let mul_ba = SimdOps::mul_f64(&b, &a);
            
            assert_eq!(mul_ab.len(), mul_ba.len(), "Length should be equal");
            for (i, (x, y)) in mul_ab.iter().zip(mul_ba.iter()).enumerate() {
                assert!((x - y).abs() < 1e-10, 
                    "Multiplication should be commutative at index {}: a: {:?}, b: {:?}", i, a, b);
            }
        }
    }

    #[test]
    fn test_mul_associativity() {
        // Multiplication should be associative: (a * b) * c = a * (b * c)
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = vec![7.0, 8.0, 9.0];
        
        let ab = SimdOps::mul_f64(&a, &b);
        let abc = SimdOps::mul_f64(&ab, &c);
        
        let bc = SimdOps::mul_f64(&b, &c);
        let abc2 = SimdOps::mul_f64(&a, &bc);
        
        assert_eq!(abc.len(), abc2.len(), "Length should be equal");
        for (i, (x, y)) in abc.iter().zip(abc2.iter()).enumerate() {
            assert!((x - y).abs() < 1e-10, 
                "Multiplication should be associative at index {}", i);
        }
    }

    #[test]
    fn test_string_operation_properties() {
        // String operations should be idempotent: to_upper(to_upper(s)) = to_upper(s)
        let test_strings = vec![
            "hello world",
            "HELLO WORLD",
            "Hello World",
            "hElLo WoRlD",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ];

        for string in test_strings {
            let upper1 = SimdStringOps::to_uppercase_simd(string);
            let upper2 = SimdStringOps::to_uppercase_simd(&upper1);
            assert_eq!(upper1, upper2, "to_uppercase should be idempotent for: '{}'", string);
            
            let lower1 = SimdStringOps::to_lowercase_simd(string);
            let lower2 = SimdStringOps::to_lowercase_simd(&lower1);
            assert_eq!(lower1, lower2, "to_lowercase should be idempotent for: '{}'", string);
        }
    }

    #[test]
    fn test_string_contains_properties() {
        // Contains should be reflexive: s.contains(s) = true (for non-empty s)
        let test_strings = vec![
            "a",
            "ab",
            "abc",
            "hello",
            "hello world",
            "abcdefghijklmnopqrstuvwxyz",
        ];

        for string in test_strings {
            if !string.is_empty() {
                assert!(SimdStringOps::contains_simd(string, string), 
                    "String should contain itself: '{}'", string);
            }
        }
    }

    #[test]
    fn test_string_contains_transitivity() {
        // If s1 contains s2 and s2 contains s3, then s1 contains s3
        let test_cases = vec![
            ("hello world", "hello", "he"),
            ("abcdefghijklmnopqrstuvwxyz", "def", "ef"),
            ("hello world", "world", "or"),
        ];

        for (s1, s2, s3) in test_cases {
            let s1_contains_s2 = SimdStringOps::contains_simd(s1, s2);
            let s2_contains_s3 = SimdStringOps::contains_simd(s2, s3);
            let s1_contains_s3 = SimdStringOps::contains_simd(s1, s3);
            
            if s1_contains_s2 && s2_contains_s3 {
                assert!(s1_contains_s3, 
                    "Contains should be transitive: '{}' contains '{}' and '{}' contains '{}', so '{}' should contain '{}'", 
                    s1, s2, s2, s3, s1, s3);
            }
        }
    }

    #[test]
    fn test_simd_type_consistency() {
        // SIMD type should be consistent across multiple calls
        let simd_type1 = get_best_simd_type();
        let simd_type2 = get_best_simd_type();
        let simd_type3 = get_best_simd_type();
        
        assert_eq!(simd_type1, simd_type2, "SIMD type should be consistent");
        assert_eq!(simd_type2, simd_type3, "SIMD type should be consistent");
    }

    #[test]
    fn test_cpu_capabilities_consistency() {
        // CPU capabilities should be consistent across multiple calls
        let caps1 = get_simd_capabilities();
        let caps2 = get_simd_capabilities();
        let caps3 = get_simd_capabilities();
        
        assert_eq!(caps1.avx2, caps2.avx2, "AVX2 detection should be consistent");
        assert_eq!(caps1.neon, caps2.neon, "NEON detection should be consistent");
        assert_eq!(caps1.sse2, caps2.sse2, "SSE2 detection should be consistent");
        assert_eq!(caps1.sse4_1, caps2.sse4_1, "SSE4.1 detection should be consistent");
        
        assert_eq!(caps2.avx2, caps3.avx2, "AVX2 detection should be consistent");
        assert_eq!(caps2.neon, caps3.neon, "NEON detection should be consistent");
        assert_eq!(caps2.sse2, caps3.sse2, "SSE2 detection should be consistent");
        assert_eq!(caps2.sse4_1, caps3.sse4_1, "SSE4.1 detection should be consistent");
    }

    #[test]
    fn test_simd_operations_deterministic() {
        // SIMD operations should be deterministic
        let data = (0..1000).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
        
        let results: Vec<f64> = (0..100).map(|_| SimdOps::sum_f64(&data)).collect();
        
        // All results should be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i], "SIMD operations should be deterministic");
        }
    }

    #[test]
    fn test_simd_operations_preserve_length() {
        // Vector operations should preserve length
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        
        let add_result = SimdOps::add_f64(&a, &b);
        let mul_result = SimdOps::mul_f64(&a, &b);
        
        assert_eq!(add_result.len(), a.len(), "Add should preserve length");
        assert_eq!(mul_result.len(), a.len(), "Mul should preserve length");
    }

    #[test]
    fn test_simd_operations_handle_edge_cases() {
        // SIMD operations should handle edge cases correctly
        let edge_cases = vec![
            vec![], // Empty
            vec![0.0], // Single element
            vec![0.0, 0.0], // Two identical elements
            vec![f64::NAN], // NaN
            vec![f64::INFINITY], // Infinity
            vec![f64::NEG_INFINITY], // Negative infinity
        ];

        for data in edge_cases {
            // These operations should not panic
            let _ = SimdOps::sum_f64(&data);
            let _ = SimdOps::mean_f64(&data);
            let _ = SimdOps::min_max_f64(&data);
            let _ = SimdOps::variance_f64(&data);
            let _ = SimdOps::std_dev_f64(&data);
        }
    }
}
