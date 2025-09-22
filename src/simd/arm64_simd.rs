#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

// ARM64 NEON SIMD operations for Apple Silicon and ARM64 platforms
#[cfg(target_arch = "aarch64")]
pub struct Arm64SimdOps;

#[cfg(target_arch = "aarch64")]
impl Arm64SimdOps {
    // NEON sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        if data.len() < 2 {
            return data.iter().sum();
        }

        unsafe {
            let mut sum = vdupq_n_s64(0);
            let chunks = data.chunks_exact(2);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let values = vld1q_s64(chunk.as_ptr());
                sum = vaddq_s64(sum, values);
            }

            // Extract sum from NEON register
            let mut result = [0i64; 2];
            vst1q_s64(result.as_mut_ptr(), sum);
            let simd_sum = result[0] + result[1];

            // Add remainder
            simd_sum + remainder.iter().sum::<i64>()
        }
    }

    // NEON sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return data.iter().sum();
        }

        unsafe {
            let mut sum = vdupq_n_f64(0.0);
            let chunks = data.chunks_exact(2);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let values = vld1q_f64(chunk.as_ptr());
                sum = vaddq_f64(sum, values);
            }

            // Extract sum from NEON register
            let mut result = [0.0f64; 2];
            vst1q_f64(result.as_mut_ptr(), sum);
            let simd_sum = result[0] + result[1];

            // Add remainder
            simd_sum + remainder.iter().sum::<f64>()
        }
    }

    // NEON mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        let sum = Self::sum_f64(data);
        sum / data.len() as f64
    }

    // NEON min/max for i64 vectors (scalar fallback - NEON doesn't have i64 min/max)
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        if data.is_empty() {
            return (0, 0);
        }
        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();
        (*min, *max)
    }

    // NEON min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        if data.is_empty() {
            return (0.0, 0.0);
        }

        if data.len() < 2 {
            let min = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            return (*min, *max);
        }

        unsafe {
            let mut min_val = vdupq_n_f64(f64::MAX);
            let mut max_val = vdupq_n_f64(f64::MIN);
            let chunks = data.chunks_exact(2);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let values = vld1q_f64(chunk.as_ptr());
                min_val = vminq_f64(min_val, values);
                max_val = vmaxq_f64(max_val, values);
            }

            // Extract min/max from NEON registers
            let mut min_result = [0.0f64; 2];
            let mut max_result = [0.0f64; 2];
            vst1q_f64(min_result.as_mut_ptr(), min_val);
            vst1q_f64(max_result.as_mut_ptr(), max_val);

            let simd_min = min_result.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let simd_max = max_result.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

            // Check remainder
            let remainder_min = remainder.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&simd_min);
            let remainder_max = remainder.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&simd_max);

            (simd_min.min(*remainder_min), simd_max.max(*remainder_max))
        }
    }

    // NEON variance calculation
    pub fn variance_f64(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = Self::mean_f64(data);
        let variance = Self::sum_f64(&data.iter().map(|x| (x - mean).powi(2)).collect::<Vec<f64>>());
        variance / (data.len() - 1) as f64
    }

    // NEON standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        Self::variance_f64(data).sqrt()
    }

    // NEON dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() || a.len() < 2 {
            return a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        }

        unsafe {
            let mut sum = vdupq_n_f64(0.0);
            let chunks = a.chunks_exact(2);
            let remainder = chunks.remainder();

            for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(2)) {
                let values_a = vld1q_f64(chunk_a.as_ptr());
                let values_b = vld1q_f64(chunk_b.as_ptr());
                let product = vmulq_f64(values_a, values_b);
                sum = vaddq_f64(sum, product);
            }

            // Extract sum from NEON register
            let mut result = [0.0f64; 2];
            vst1q_f64(result.as_mut_ptr(), sum);
            let simd_sum = result[0] + result[1];

            // Add remainder
            let remainder_sum: f64 = remainder.iter()
                .zip(b.chunks_exact(2).remainder().iter())
                .map(|(x, y)| x * y)
                .sum();

            simd_sum + remainder_sum
        }
    }

    // NEON element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }

        if a.len() < 2 {
            return a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
        }

        unsafe {
            let mut result = Vec::with_capacity(a.len());
            let chunks = a.chunks_exact(2);
            let remainder = chunks.remainder();

            for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(2)) {
                let values_a = vld1q_f64(chunk_a.as_ptr());
                let values_b = vld1q_f64(chunk_b.as_ptr());
                let sum = vaddq_f64(values_a, values_b);

                let mut chunk_result = [0.0f64; 2];
                vst1q_f64(chunk_result.as_mut_ptr(), sum);
                result.extend_from_slice(&chunk_result);
            }

            // Handle remainder
            for (x, y) in remainder.iter().zip(b.chunks_exact(2).remainder().iter()) {
                result.push(x + y);
            }

            result
        }
    }

    // NEON element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }

        if a.len() < 2 {
            return a.iter().zip(b.iter()).map(|(x, y)| x * y).collect();
        }

        unsafe {
            let mut result = Vec::with_capacity(a.len());
            let chunks = a.chunks_exact(2);
            let remainder = chunks.remainder();

            for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(2)) {
                let values_a = vld1q_f64(chunk_a.as_ptr());
                let values_b = vld1q_f64(chunk_b.as_ptr());
                let product = vmulq_f64(values_a, values_b);

                let mut chunk_result = [0.0f64; 2];
                vst1q_f64(chunk_result.as_mut_ptr(), product);
                result.extend_from_slice(&chunk_result);
            }

            // Handle remainder
            for (x, y) in remainder.iter().zip(b.chunks_exact(2).remainder().iter()) {
                result.push(x * y);
            }

            result
        }
    }
}

// ARM64 string operations with NEON
#[cfg(target_arch = "aarch64")]
pub struct Arm64StringOps;

#[cfg(target_arch = "aarch64")]
impl Arm64StringOps {
    // NEON string upper case
    pub fn to_uppercase_simd(input: &str) -> String {
        if input.len() < 16 {
            return input.to_uppercase();
        }

        unsafe {
            let mut result = String::with_capacity(input.len());
            let bytes = input.as_bytes();
            let chunks = bytes.chunks_exact(16);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let values = vld1q_u8(chunk.as_ptr());
                let upper = vsubq_u8(values, vdupq_n_u8(32));
                let mut result_chunk = [0u8; 16];
                vst1q_u8(result_chunk.as_mut_ptr(), upper);
                result.push_str(std::str::from_utf8_unchecked(&result_chunk));
            }

            // Handle remainder
            for &byte in remainder {
                result.push((byte as char).to_uppercase().next().unwrap());
            }

            result
        }
    }

    // NEON string lower case
    pub fn to_lowercase_simd(input: &str) -> String {
        if input.len() < 16 {
            return input.to_lowercase();
        }

        unsafe {
            let mut result = String::with_capacity(input.len());
            let bytes = input.as_bytes();
            let chunks = bytes.chunks_exact(16);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let values = vld1q_u8(chunk.as_ptr());
                let lower = vaddq_u8(values, vdupq_n_u8(32));
                let mut result_chunk = [0u8; 16];
                vst1q_u8(result_chunk.as_mut_ptr(), lower);
                result.push_str(std::str::from_utf8_unchecked(&result_chunk));
            }

            // Handle remainder
            for &byte in remainder {
                result.push((byte as char).to_lowercase().next().unwrap());
            }

            result
        }
    }

    // NEON string contains
    pub fn contains_simd(haystack: &str, needle: &str) -> bool {
        if needle.is_empty() {
            return true;
        }

        if haystack.len() < needle.len() {
            return false;
        }

        let haystack_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let needle_len = needle_bytes.len();

        if needle_len < 16 {
            return haystack_bytes.windows(needle_len).any(|window| window == needle_bytes);
        }

        unsafe {
            let needle_simd = vld1q_u8(needle_bytes.as_ptr());
            
            for i in 0..=haystack_bytes.len() - needle_len {
                let haystack_simd = vld1q_u8(haystack_bytes.as_ptr().add(i));
                let cmp = vceqq_u8(needle_simd, haystack_simd);
                let mask = vget_lane_u64(vreinterpret_u64_u8(vget_low_u8(cmp)), 0);
                
                if mask == 0xFFFFFFFFFFFFFFFF {
                    return true;
                }
            }
        }

        false
    }
}
