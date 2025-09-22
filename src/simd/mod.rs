// Cross-platform SIMD operations with automatic feature detection
// Uses architecture-specific SIMD when available, falls back to scalar operations

// Architecture-specific modules
#[cfg(target_arch = "x86_64")]
mod x86_64_simd;

#[cfg(target_arch = "aarch64")]
mod arm64_simd;

mod cpu_features;
mod scalar_fallback;

use cpu_features::{SimdCapabilities, SimdType, get_simd_capabilities, get_best_simd_type};

pub struct SimdOps;

impl SimdOps {
    // SIMD-optimized sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::sum_i64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::sum_i64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::sum_i64(data)
            }
        }
    }

    // SIMD-optimized sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::sum_f64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::sum_f64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::sum_f64(data)
            }
        }
    }

    // SIMD-optimized mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::mean_f64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::mean_f64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::mean_f64(data)
            }
        }
    }

    // SIMD-optimized min/max for i64 vectors
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::min_max_i64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::min_max_i64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::min_max_i64(data)
            }
        }
    }

    // SIMD-optimized min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::min_max_f64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::min_max_f64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::min_max_f64(data)
            }
        }
    }

    // SIMD-optimized variance calculation
    pub fn variance_f64(data: &[f64]) -> f64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::variance_f64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::variance_f64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::variance_f64(data)
            }
        }
    }

    // SIMD-optimized standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::std_dev_f64(data)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::std_dev_f64(data)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::std_dev_f64(data)
            }
        }
    }

    // SIMD-optimized dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::dot_product_f64(a, b)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::dot_product_f64(a, b)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::dot_product_f64(a, b)
            }
        }
    }

    // SIMD-optimized element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::add_f64(a, b)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::add_f64(a, b)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::add_f64(a, b)
            }
        }
    }

    // SIMD-optimized element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64SimdOps;
                X86_64SimdOps::mul_f64(a, b)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64SimdOps;
                Arm64SimdOps::mul_f64(a, b)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarOps;
                ScalarOps::mul_f64(a, b)
            }
        }
    }

    // Get SIMD capabilities
    pub fn get_capabilities() -> SimdCapabilities {
        get_simd_capabilities()
    }

    // Get best SIMD type available
    pub fn get_simd_type() -> SimdType {
        get_best_simd_type()
    }
}

// Cross-platform SIMD string operations
pub struct SimdStringOps;

impl SimdStringOps {
    // SIMD-optimized string upper case
    pub fn to_uppercase_simd(input: &str) -> String {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64StringOps;
                X86_64StringOps::to_uppercase_simd(input)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64StringOps;
                Arm64StringOps::to_uppercase_simd(input)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarStringOps;
                ScalarStringOps::to_uppercase_simd(input)
            }
        }
    }

    // SIMD-optimized string lower case
    pub fn to_lowercase_simd(input: &str) -> String {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64StringOps;
                X86_64StringOps::to_lowercase_simd(input)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64StringOps;
                Arm64StringOps::to_lowercase_simd(input)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarStringOps;
                ScalarStringOps::to_lowercase_simd(input)
            }
        }
    }

    // SIMD-optimized string contains
    pub fn contains_simd(haystack: &str, needle: &str) -> bool {
        match get_best_simd_type() {
            #[cfg(target_arch = "x86_64")]
            SimdType::AVX2 => {
                use crate::simd::x86_64_simd::X86_64StringOps;
                X86_64StringOps::contains_simd(haystack, needle)
            }
            #[cfg(target_arch = "aarch64")]
            SimdType::NEON => {
                use crate::simd::arm64_simd::Arm64StringOps;
                Arm64StringOps::contains_simd(haystack, needle)
            }
            _ => {
                use crate::simd::scalar_fallback::ScalarStringOps;
                ScalarStringOps::contains_simd(haystack, needle)
            }
        }
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