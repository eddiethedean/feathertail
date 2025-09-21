#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// SIMD-enabled operations for numerical columns
pub struct SimdOps;

impl SimdOps {
    // SIMD sum for i64 vectors
    pub fn sum_i64(data: &[i64]) -> i64 {
        if data.len() < 8 {
            return data.iter().sum();
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut sum = _mm256_setzero_si256();
                let chunks = data.chunks_exact(4);
                let remainder = chunks.remainder();

                for chunk in chunks {
                    let values = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                    sum = _mm256_add_epi64(sum, values);
                }

                // Extract sum from SIMD register
                let mut result = [0i64; 4];
                _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, sum);
                let simd_sum = result[0] + result[1] + result[2] + result[3];

                // Add remainder
                simd_sum + remainder.iter().sum::<i64>()
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            data.iter().sum()
        }
    }

    // SIMD sum for f64 vectors
    pub fn sum_f64(data: &[f64]) -> f64 {
        if data.len() < 4 {
            return data.iter().sum();
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut sum = _mm256_setzero_pd();
                let chunks = data.chunks_exact(4);
                let remainder = chunks.remainder();

                for chunk in chunks {
                    let values = _mm256_loadu_pd(chunk.as_ptr());
                    sum = _mm256_add_pd(sum, values);
                }

                // Extract sum from SIMD register
                let mut result = [0.0f64; 4];
                _mm256_storeu_pd(result.as_mut_ptr(), sum);
                let simd_sum = result[0] + result[1] + result[2] + result[3];

                // Add remainder
                simd_sum + remainder.iter().sum::<f64>()
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            data.iter().sum()
        }
    }

    // SIMD mean for f64 vectors
    pub fn mean_f64(data: &[f64]) -> f64 {
        let sum = Self::sum_f64(data);
        sum / data.len() as f64
    }

    // SIMD min/max for i64 vectors
    pub fn min_max_i64(data: &[i64]) -> (i64, i64) {
        if data.is_empty() {
            return (0, 0);
        }

        if data.len() < 4 {
            let min = data.iter().min().unwrap();
            let max = data.iter().max().unwrap();
            return (*min, *max);
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut min_val = _mm256_set1_epi64x(i64::MAX);
                let mut max_val = _mm256_set1_epi64x(i64::MIN);
                let chunks = data.chunks_exact(4);
                let remainder = chunks.remainder();

                for chunk in chunks {
                    let values = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
                    min_val = _mm256_min_epi64(min_val, values);
                    max_val = _mm256_max_epi64(max_val, values);
                }

                // Extract min/max from SIMD registers
                let mut min_result = [0i64; 4];
                let mut max_result = [0i64; 4];
                _mm256_storeu_si256(min_result.as_mut_ptr() as *mut __m256i, min_val);
                _mm256_storeu_si256(max_result.as_mut_ptr() as *mut __m256i, max_val);

                let simd_min = min_result.iter().min().unwrap();
                let simd_max = max_result.iter().max().unwrap();

                // Check remainder
                let remainder_min = remainder.iter().min().unwrap_or(*simd_min);
                let remainder_max = remainder.iter().max().unwrap_or(*simd_max);

                (*simd_min.min(remainder_min), *simd_max.max(remainder_max))
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let min = data.iter().min().unwrap();
            let max = data.iter().max().unwrap();
            (*min, *max)
        }
    }

    // SIMD min/max for f64 vectors
    pub fn min_max_f64(data: &[f64]) -> (f64, f64) {
        if data.is_empty() {
            return (0.0, 0.0);
        }

        if data.len() < 4 {
            let min = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            return (*min, *max);
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut min_val = _mm256_set1_pd(f64::MAX);
                let mut max_val = _mm256_set1_pd(f64::MIN);
                let chunks = data.chunks_exact(4);
                let remainder = chunks.remainder();

                for chunk in chunks {
                    let values = _mm256_loadu_pd(chunk.as_ptr());
                    min_val = _mm256_min_pd(min_val, values);
                    max_val = _mm256_max_pd(max_val, values);
                }

                // Extract min/max from SIMD registers
                let mut min_result = [0.0f64; 4];
                let mut max_result = [0.0f64; 4];
                _mm256_storeu_pd(min_result.as_mut_ptr(), min_val);
                _mm256_storeu_pd(max_result.as_mut_ptr(), max_val);

                let simd_min = min_result.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                let simd_max = max_result.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

                // Check remainder
                let remainder_min = remainder.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(*simd_min);
                let remainder_max = remainder.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(*simd_max);

                (*simd_min.min(remainder_min), *simd_max.max(remainder_max))
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let min = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            (*min, *max)
        }
    }

    // SIMD variance calculation
    pub fn variance_f64(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = Self::mean_f64(data);
        let variance = Self::sum_f64(&data.iter().map(|x| (x - mean).powi(2)).collect::<Vec<f64>>());
        variance / (data.len() - 1) as f64
    }

    // SIMD standard deviation
    pub fn std_dev_f64(data: &[f64]) -> f64 {
        Self::variance_f64(data).sqrt()
    }

    // SIMD dot product for two f64 vectors
    pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() || a.len() < 4 {
            return a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut sum = _mm256_setzero_pd();
                let chunks = a.chunks_exact(4);
                let remainder = chunks.remainder();

                for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(4)) {
                    let values_a = _mm256_loadu_pd(chunk_a.as_ptr());
                    let values_b = _mm256_loadu_pd(chunk_b.as_ptr());
                    let product = _mm256_mul_pd(values_a, values_b);
                    sum = _mm256_add_pd(sum, product);
                }

                // Extract sum from SIMD register
                let mut result = [0.0f64; 4];
                _mm256_storeu_pd(result.as_mut_ptr(), sum);
                let simd_sum = result[0] + result[1] + result[2] + result[3];

                // Add remainder
                let remainder_sum: f64 = remainder.iter()
                    .zip(b.chunks_exact(4).remainder().iter())
                    .map(|(x, y)| x * y)
                    .sum();

                simd_sum + remainder_sum
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
        }
    }

    // SIMD element-wise addition
    pub fn add_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }

        if a.len() < 4 {
            return a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut result = Vec::with_capacity(a.len());
                let chunks = a.chunks_exact(4);
                let remainder = chunks.remainder();

                for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(4)) {
                    let values_a = _mm256_loadu_pd(chunk_a.as_ptr());
                    let values_b = _mm256_loadu_pd(chunk_b.as_ptr());
                    let sum = _mm256_add_pd(values_a, values_b);

                    let mut chunk_result = [0.0f64; 4];
                    _mm256_storeu_pd(chunk_result.as_mut_ptr(), sum);
                    result.extend_from_slice(&chunk_result);
                }

                // Handle remainder
                for (x, y) in remainder.iter().zip(b.chunks_exact(4).remainder().iter()) {
                    result.push(x + y);
                }

                result
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
        }
    }

    // SIMD element-wise multiplication
    pub fn mul_f64(a: &[f64], b: &[f64]) -> Vec<f64> {
        if a.len() != b.len() {
            panic!("Vectors must have the same length");
        }

        if a.len() < 4 {
            return a.iter().zip(b.iter()).map(|(x, y)| x * y).collect();
        }

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut result = Vec::with_capacity(a.len());
                let chunks = a.chunks_exact(4);
                let remainder = chunks.remainder();

                for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(4)) {
                    let values_a = _mm256_loadu_pd(chunk_a.as_ptr());
                    let values_b = _mm256_loadu_pd(chunk_b.as_ptr());
                    let product = _mm256_mul_pd(values_a, values_b);

                    let mut chunk_result = [0.0f64; 4];
                    _mm256_storeu_pd(chunk_result.as_mut_ptr(), product);
                    result.extend_from_slice(&chunk_result);
                }

                // Handle remainder
                for (x, y) in remainder.iter().zip(b.chunks_exact(4).remainder().iter()) {
                    result.push(x * y);
                }

                result
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
        }
    }
}

// SIMD string operations
pub struct SimdStringOps;

impl SimdStringOps {
    // SIMD-accelerated string upper case
    pub fn to_upper_simd(input: &str) -> String {
        if input.len() < 16 {
            return input.to_uppercase();
        }

        #[cfg(target_arch = "x86_64")]
        {
            let mut result = String::with_capacity(input.len());
            let bytes = input.as_bytes();
            let chunks = bytes.chunks_exact(16);
            let remainder = chunks.remainder();

            for chunk in chunks {
                unsafe {
                    let values = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                    let upper = _mm_sub_epi8(values, _mm_set1_epi8(32));
                    let mut result_chunk = [0u8; 16];
                    _mm_storeu_si128(result_chunk.as_mut_ptr() as *mut __m128i, upper);
                    result.push_str(std::str::from_utf8_unchecked(&result_chunk));
                }
            }

            // Handle remainder
            for &byte in remainder {
                result.push((byte as char).to_uppercase().next().unwrap());
            }

            result
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            input.to_uppercase()
        }
    }

    // SIMD-accelerated string contains
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

        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let needle_simd = _mm_loadu_si128(needle_bytes.as_ptr() as *const __m128i);
                
                for i in 0..=haystack_bytes.len() - needle_len {
                    let haystack_simd = _mm_loadu_si128(haystack_bytes.as_ptr().add(i) as *const __m128i);
                    let cmp = _mm_cmpeq_epi8(needle_simd, haystack_simd);
                    let mask = _mm_movemask_epi8(cmp);
                    
                    if mask == 0xFFFF {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// SIMD performance benchmarks
pub struct SimdBenchmarks;

impl SimdBenchmarks {
    pub fn benchmark_sum_i64(data: &[i64], iterations: usize) -> (i64, f64) {
        let start = std::time::Instant::now();
        let mut result = 0;
        
        for _ in 0..iterations {
            result = SimdOps::sum_i64(data);
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_secs_f64() / iterations as f64;
        
        (result, avg_time)
    }

    pub fn benchmark_sum_f64(data: &[f64], iterations: usize) -> (f64, f64) {
        let start = std::time::Instant::now();
        let mut result = 0.0;
        
        for _ in 0..iterations {
            result = SimdOps::sum_f64(data);
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_secs_f64() / iterations as f64;
        
        (result, avg_time)
    }

    pub fn benchmark_min_max_i64(data: &[i64], iterations: usize) -> ((i64, i64), f64) {
        let start = std::time::Instant::now();
        let mut result = (0, 0);
        
        for _ in 0..iterations {
            result = SimdOps::min_max_i64(data);
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_secs_f64() / iterations as f64;
        
        (result, avg_time)
    }
}
