// Simple fallback implementations for numerical operations
// These replace the SIMD implementations to ensure cross-platform compatibility

pub struct SimdOps;

impl SimdOps {
    // Simple sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        data.iter().sum()
    }

    // Simple sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        data.iter().sum()
    }

    // Simple mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let sum = Self::sum_f64(data);
        sum / data.len() as f64
    }

    // Simple min/max for i64 vectors
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        if data.is_empty() {
            return (0, 0);
        }
        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();
        (*min, *max)
    }

    // Simple min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        if data.is_empty() {
            return (0.0, 0.0);
        }
        let min = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        (*min, *max)
    }

    // Simple variance calculation
    pub fn variance_f64(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = Self::mean_f64(data);
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>();
        variance / (data.len() - 1) as f64
    }

    // Simple standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        Self::variance_f64(data).sqrt()
    }

    // Simple dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    // Simple element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }

    // Simple element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
    }
}

// Simple string operations
pub struct SimdStringOps;

impl SimdStringOps {
    // Simple string upper case
    pub fn to_uppercase_simd(input: &str) -> String {
        input.to_uppercase()
    }

    // Simple string lower case
    pub fn to_lowercase_simd(input: &str) -> String {
        input.to_lowercase()
    }

    // Simple string contains
    pub fn contains_simd(haystack: &str, needle: &str) -> bool {
        haystack.contains(needle)
    }
}

// Simple benchmarks
pub struct SimdBenchmarks;

impl SimdBenchmarks {
    pub fn benchmark_sum_i64(data: &[i64]) -> i64 {
        SimdOps::sum_i64(data)
    }

    pub fn benchmark_sum_f64(data: &[f64]) -> f64 {
        SimdOps::sum_f64(data)
    }

    pub fn benchmark_min_max_i64(data: &[i64]) -> (i64, i64) {
        SimdOps::min_max_i64(data)
    }

    pub fn benchmark_min_max_f64(data: &[f64]) -> (f64, f64) {
        SimdOps::min_max_f64(data)
    }
}