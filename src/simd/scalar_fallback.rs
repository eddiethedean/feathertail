// Scalar fallback implementations for platforms without SIMD support
// These provide the same interface as SIMD operations but use standard scalar code

pub struct ScalarOps;

impl ScalarOps {
    // Scalar sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        data.iter().sum()
    }

    // Scalar sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        data.iter().sum()
    }

    // Scalar mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let sum = Self::sum_f64(data);
        sum / data.len() as f64
    }

    // Scalar min/max for i64 vectors
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        if data.is_empty() {
            return (0, 0);
        }
        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();
        (*min, *max)
    }

    // Scalar min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        if data.is_empty() {
            return (0.0, 0.0);
        }
        let min = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        (*min, *max)
    }

    // Scalar variance calculation
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

    // Scalar standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        Self::variance_f64(data).sqrt()
    }

    // Scalar dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    // Scalar element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }

    // Scalar element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }
        a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
    }
}

// Scalar string operations
pub struct ScalarStringOps;

impl ScalarStringOps {
    // Scalar string upper case
    pub fn to_uppercase_simd(input: &str) -> String {
        input.to_uppercase()
    }

    // Scalar string lower case
    pub fn to_lowercase_simd(input: &str) -> String {
        input.to_lowercase()
    }

    // Scalar string contains
    pub fn contains_simd(haystack: &str, needle: &str) -> bool {
        haystack.contains(needle)
    }
}
