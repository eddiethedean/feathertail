// Performance benchmarks for SIMD operations
use std::time::{Duration, Instant};
use std::collections::HashMap;
    use crate::simd::{SimdOps, SimdStringOps, SimdType, get_best_simd_type};
    use crate::simd::scalar_fallback::ScalarOps;

pub struct BenchmarkResult {
    pub operation: String,
    pub data_size: usize,
    pub iterations: usize,
    pub total_time: Duration,
    pub avg_time_per_op: Duration,
    pub ops_per_second: f64,
    pub simd_type: String,
}

pub struct SimdBenchmarker {
    results: Vec<BenchmarkResult>,
}

impl SimdBenchmarker {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn run_all_benchmarks(&mut self) {
        println!("🚀 Running SIMD Performance Benchmarks");
        println!("=====================================");
        
        // Test different data sizes
        let sizes = vec![100, 1000, 10000, 100000, 1000000];
        
        for size in sizes {
            self.benchmark_numerical_operations(size);
            self.benchmark_string_operations(size);
        }
        
        self.print_summary();
    }

    fn benchmark_numerical_operations(&mut self, size: usize) {
        let data_i64 = self.generate_test_data_i64(size);
        let data_f64 = self.generate_test_data_f64(size);
        let iterations = self.get_iterations_for_size(size);
        
        // Benchmark sum operations
        self.benchmark_operation("sum_i64", size, iterations, || {
            SimdOps::sum_i64(&data_i64)
        });
        
        self.benchmark_operation("sum_f64", size, iterations, || {
            SimdOps::sum_f64(&data_f64)
        });
        
        // Benchmark mean operations
        self.benchmark_operation("mean_f64", size, iterations, || {
            SimdOps::mean_f64(&data_f64)
        });
        
        // Benchmark min/max operations
        self.benchmark_operation("min_max_i64", size, iterations, || {
            SimdOps::min_max_i64(&data_i64)
        });
        
        self.benchmark_operation("min_max_f64", size, iterations, || {
            SimdOps::min_max_f64(&data_f64)
        });
        
        // Benchmark variance operations
        self.benchmark_operation("variance_f64", size, iterations, || {
            SimdOps::variance_f64(&data_f64)
        });
        
        self.benchmark_operation("std_dev_f64", size, iterations, || {
            SimdOps::std_dev_f64(&data_f64)
        });
        
        // Benchmark vector operations
        let data_f64_b = self.generate_test_data_f64(size);
        self.benchmark_operation("dot_product_f64", size, iterations, || {
            SimdOps::dot_product_f64(&data_f64, &data_f64_b)
        });
        
        self.benchmark_operation("add_f64", size, iterations, || {
            SimdOps::add_f64(&data_f64, &data_f64_b)
        });
        
        self.benchmark_operation("mul_f64", size, iterations, || {
            SimdOps::mul_f64(&data_f64, &data_f64_b)
        });
    }

    fn benchmark_string_operations(&mut self, size: usize) {
        let test_strings = self.generate_test_strings(size);
        let iterations = self.get_iterations_for_size(size);
        
        // Benchmark string case operations
        for (i, string) in test_strings.iter().enumerate() {
            if i >= 5 { break; } // Limit to first 5 strings for performance
            
            self.benchmark_operation("to_uppercase", size, iterations, || {
                SimdStringOps::to_uppercase_simd(string)
            });
            
            self.benchmark_operation("to_lowercase", size, iterations, || {
                SimdStringOps::to_lowercase_simd(string)
            });
        }
        
        // Benchmark string contains
        if !test_strings.is_empty() {
            let haystack = &test_strings[0];
            let needle = "test";
            self.benchmark_operation("contains", size, iterations, || {
                SimdStringOps::contains_simd(haystack, needle)
            });
        }
    }

    fn benchmark_operation<F, R>(&mut self, operation: &str, data_size: usize, iterations: usize, f: F)
    where
        F: Fn() -> R,
    {
        // Warm up
        for _ in 0..10 {
            let _ = f();
        }
        
        // Benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = f();
        }
        let total_time = start.elapsed();
        
        let avg_time_per_op = total_time / iterations as u32;
        let ops_per_second = iterations as f64 / total_time.as_secs_f64();
        
        let simd_type = match get_best_simd_type() {
            SimdType::AVX2 => "AVX2".to_string(),
            SimdType::NEON => "NEON".to_string(),
            SimdType::Scalar => "Scalar".to_string(),
        };
        
        let result = BenchmarkResult {
            operation: operation.to_string(),
            data_size,
            iterations,
            total_time,
            avg_time_per_op,
            ops_per_second,
            simd_type: simd_type.clone(),
        };
        
        self.results.push(result);
        
        println!("✅ {} ({}): {:.2} ops/sec, {:.2}μs/op, {} elements", 
                operation, simd_type, ops_per_second, 
                avg_time_per_op.as_micros() as f64, data_size);
    }

    fn get_iterations_for_size(&self, size: usize) -> usize {
        match size {
            0..=1000 => 10000,
            1001..=10000 => 1000,
            10001..=100000 => 100,
            100001..=1000000 => 10,
            _ => 1,
        }
    }

    fn generate_test_data_i64(&self, size: usize) -> Vec<i64> {
        (0..size).map(|i| (i as i64) * 3 - 1000).collect()
    }

    fn generate_test_data_f64(&self, size: usize) -> Vec<f64> {
        (0..size).map(|i| (i as f64) * 0.5 + 100.0).collect()
    }

    fn generate_test_strings(&self, size: usize) -> Vec<String> {
        let base_strings = vec![
            "hello world",
            "HELLO WORLD", 
            "Hello World",
            "hElLo WoRlD",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "1234567890",
            "!@#$%^&*()",
        ];
        
        let mut result = Vec::new();
        for i in 0..size {
            result.push(format!("{}_{}", base_strings[i % base_strings.len()], i));
        }
        result
    }

    fn print_summary(&self) {
        println!("\n📊 Benchmark Summary");
        println!("===================");
        
        // Group results by operation
        let mut grouped: HashMap<String, Vec<&BenchmarkResult>> = HashMap::new();
        for result in &self.results {
            grouped.entry(result.operation.clone()).or_insert_with(Vec::new).push(result);
        }
        
        for (operation, results) in grouped {
            println!("\n🔍 {}", operation);
            println!("Size\t\tOps/sec\t\tTime/op\t\tSIMD Type");
            println!("----\t\t-------\t\t-------\t\t---------");
            
            for result in results {
                println!("{}\t\t{:.0}\t\t{:.2}μs\t\t{}", 
                        result.data_size, result.ops_per_second, 
                        result.avg_time_per_op.as_micros() as f64, result.simd_type);
            }
        }
        
        // Performance analysis
        self.analyze_performance();
    }

    fn analyze_performance(&self) {
        println!("\n🔬 Performance Analysis");
        println!("=====================");
        
        let simd_type = match get_best_simd_type() {
            SimdType::AVX2 => "AVX2",
            SimdType::NEON => "NEON", 
            SimdType::Scalar => "Scalar",
        };
        
        println!("SIMD Type: {}", simd_type);
        
        // Find the best performing operations
        let mut best_ops_per_second = 0.0;
        let mut best_operation = "";
        
        for result in &self.results {
            if result.ops_per_second > best_ops_per_second {
                best_ops_per_second = result.ops_per_second;
                best_operation = &result.operation;
            }
        }
        
        println!("Best performing operation: {} ({:.0} ops/sec)", best_operation, best_ops_per_second);
        
        // Calculate speedup estimates
        if simd_type != "Scalar" {
            println!("Expected speedup over scalar: 2-4x for numerical operations");
            println!("Expected speedup over scalar: 1.5-3x for string operations");
        } else {
            println!("Using scalar fallback - no SIMD acceleration available");
        }
    }

    pub fn get_results(&self) -> &Vec<BenchmarkResult> {
        &self.results
    }
}

// Convenience function to run benchmarks
pub fn run_simd_benchmarks() {
    let mut benchmarker = SimdBenchmarker::new();
    benchmarker.run_all_benchmarks();
}

// Micro-benchmark for specific operations
pub fn micro_benchmark<F, R>(name: &str, iterations: usize, f: F) -> BenchmarkResult
where
    F: Fn() -> R,
{
    // Warm up
    for _ in 0..10 {
        let _ = f();
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = f();
    }
    let total_time = start.elapsed();
    
    let avg_time_per_op = total_time / iterations as u32;
    let ops_per_second = iterations as f64 / total_time.as_secs_f64();
    
    let simd_type = match get_best_simd_type() {
        SimdType::AVX2 => "AVX2".to_string(),
        SimdType::NEON => "NEON".to_string(),
        SimdType::Scalar => "Scalar".to_string(),
    };
    
    BenchmarkResult {
        operation: name.to_string(),
        data_size: 0, // Not applicable for micro-benchmarks
        iterations,
        total_time,
        avg_time_per_op,
        ops_per_second,
        simd_type,
    }
}

// Compare SIMD vs Scalar performance
pub fn compare_simd_vs_scalar() {
    println!("🔄 SIMD vs Scalar Performance Comparison");
    println!("=======================================");
    
    let data_sizes = vec![1000, 10000, 100000];
    
    for size in data_sizes {
        let data = (0..size).map(|i| (i as f64) * 0.5 + 100.0).collect::<Vec<f64>>();
        let iterations = 1000;
        
        // Benchmark SIMD version
        let simd_start = Instant::now();
        for _ in 0..iterations {
            let _ = SimdOps::sum_f64(&data);
        }
        let simd_time = simd_start.elapsed();
        
        // Benchmark scalar version (using the fallback directly)
        let scalar_start = Instant::now();
        for _ in 0..iterations {
            let _ = ScalarOps::sum_f64(&data);
        }
        let scalar_time = scalar_start.elapsed();
        
        let speedup = scalar_time.as_secs_f64() / simd_time.as_secs_f64();
        
        println!("Size: {}, SIMD: {:.2}ms, Scalar: {:.2}ms, Speedup: {:.2}x", 
                size, simd_time.as_millis(), scalar_time.as_millis(), speedup);
    }
}
