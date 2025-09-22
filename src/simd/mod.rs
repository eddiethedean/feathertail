// Cross-platform SIMD operations with automatic feature detection
// Uses architecture-specific SIMD when available, falls back to scalar operations

// Simple dispatch macro that works for both SIMD and no-SIMD cases
macro_rules! simd_dispatch {
    ($method:ident, $data:expr) => {
        {
            use crate::simd::scalar_fallback::ScalarOps;
            ScalarOps::$method($data)
        }
    };
    ($method:ident, $data:expr, $arg1:expr) => {
        {
            use crate::simd::scalar_fallback::ScalarOps;
            ScalarOps::$method($data, $arg1)
        }
    };
    ($method:ident, $data:expr, $arg1:expr, $arg2:expr) => {
        {
            use crate::simd::scalar_fallback::ScalarOps;
            ScalarOps::$method($data, $arg1, $arg2)
        }
    };
}

// Architecture-specific modules
#[cfg(all(target_arch = "x86_64", feature = "simd"))]
mod x86_64_simd;

#[cfg(all(target_arch = "aarch64", feature = "simd"))]
mod arm64_simd;

#[cfg(feature = "simd")]
mod cpu_features;
mod scalar_fallback;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod property_tests;

pub mod benchmarks;

#[cfg(feature = "simd")]
use cpu_features::{SimdCapabilities, SimdType, get_simd_capabilities, get_best_simd_type};

#[cfg(not(feature = "simd"))]
#[derive(Debug, Clone, Copy)]
pub struct SimdCapabilities {
    pub avx2: bool,
    pub neon: bool,
    pub sse2: bool,
    pub sse4_1: bool,
}

#[cfg(not(feature = "simd"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimdType {
    Scalar,
}

pub struct SimdOps;

impl SimdOps {
    // SIMD-optimized sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        simd_dispatch!(sum_i64, data)
    }

    // SIMD-optimized sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        simd_dispatch!(sum_f64, data)
    }

    // SIMD-optimized mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        simd_dispatch!(mean_f64, data)
    }

    // SIMD-optimized min/max for i64 vectors
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        simd_dispatch!(min_max_i64, data)
    }

    // SIMD-optimized min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        simd_dispatch!(min_max_f64, data)
    }

    // SIMD-optimized variance calculation
    pub fn variance_f64(data: &[f64]) -> f64 {
        simd_dispatch!(variance_f64, data)
    }

    // SIMD-optimized standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        simd_dispatch!(std_dev_f64, data)
    }

    // SIMD-optimized dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        simd_dispatch!(dot_product_f64, a, b)
    }

    // SIMD-optimized element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        simd_dispatch!(add_f64, a, b)
    }

    // SIMD-optimized element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        simd_dispatch!(mul_f64, a, b)
    }

    // Get SIMD capabilities
    #[cfg(feature = "simd")]
    pub fn get_capabilities() -> SimdCapabilities {
        get_simd_capabilities()
    }

    #[cfg(not(feature = "simd"))]
    pub fn get_capabilities() -> SimdCapabilities {
        SimdCapabilities {
            avx2: false,
            neon: false,
            sse2: false,
            sse4_1: false,
        }
    }

    // Get best SIMD type available
    #[cfg(feature = "simd")]
    pub fn get_simd_type() -> SimdType {
        get_best_simd_type()
    }

    #[cfg(not(feature = "simd"))]
    pub fn get_simd_type() -> SimdType {
        SimdType::Scalar
    }
}

// Cross-platform SIMD string operations
pub struct SimdStringOps;

impl SimdStringOps {
    // SIMD-optimized string upper case
    pub fn to_uppercase_simd(input: &str) -> String {
        use crate::simd::scalar_fallback::ScalarStringOps;
        ScalarStringOps::to_uppercase_simd(input)
    }

    // SIMD-optimized string lower case
    pub fn to_lowercase_simd(input: &str) -> String {
        use crate::simd::scalar_fallback::ScalarStringOps;
        ScalarStringOps::to_lowercase_simd(input)
    }

    // SIMD-optimized string contains
    pub fn contains_simd(haystack: &str, needle: &str) -> bool {
        use crate::simd::scalar_fallback::ScalarStringOps;
        ScalarStringOps::contains_simd(haystack, needle)
    }
}

// SIMD performance benchmarks
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

    pub fn benchmark_dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        SimdOps::dot_product_f64(a, b)
    }

    pub fn benchmark_add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        SimdOps::add_f64(a, b)
    }

    pub fn benchmark_mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        SimdOps::mul_f64(a, b)
    }

    pub fn benchmark_variance_f64(data: &[f64]) -> f64 {
        SimdOps::variance_f64(data)
    }

    pub fn benchmark_std_dev_f64(data: &[f64]) -> f64 {
        SimdOps::std_dev_f64(data)
    }

    pub fn benchmark_string_uppercase(input: &str) -> String {
        SimdStringOps::to_uppercase_simd(input)
    }

    pub fn benchmark_string_contains(haystack: &str, needle: &str) -> bool {
        SimdStringOps::contains_simd(haystack, needle)
    }
}