// CPU feature detection for SIMD capabilities
use std::sync::Once;

static INIT: Once = Once::new();
static mut SIMD_CAPABILITIES: Option<SimdCapabilities> = None;

#[derive(Debug, Clone, Copy)]
pub struct SimdCapabilities {
    pub avx2: bool,
    pub neon: bool,
    pub sse2: bool,
    pub sse4_1: bool,
}

impl SimdCapabilities {
    pub fn new() -> Self {
        Self {
            avx2: false,
            neon: false,
            sse2: false,
            sse4_1: false,
        }
    }

    pub fn detect() -> Self {
        unsafe {
            INIT.call_once(|| {
                SIMD_CAPABILITIES = Some(Self::detect_cpu_features());
            });
            SIMD_CAPABILITIES.unwrap_or_else(Self::new)
        }
    }

    fn detect_cpu_features() -> Self {
        let mut capabilities = Self::new();

        #[cfg(target_arch = "x86_64")]
        {
            capabilities.avx2 = has_avx2_impl();
            capabilities.sse2 = has_sse2_impl();
            capabilities.sse4_1 = has_sse4_1_impl();
        }

        #[cfg(target_arch = "aarch64")]
        {
            capabilities.neon = has_neon_impl();
        }

        capabilities
    }

    pub fn has_simd(&self) -> bool {
        self.avx2 || self.neon
    }

    pub fn get_best_simd_type(&self) -> SimdType {
        if self.avx2 {
            SimdType::AVX2
        } else if self.neon {
            SimdType::NEON
        } else {
            SimdType::Scalar
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimdType {
    AVX2,
    NEON,
    Scalar,
}

// CPU feature detection implementations
#[cfg(target_arch = "x86_64")]
fn has_avx2_impl() -> bool {
    unsafe {
        let cpuid = std::arch::x86_64::__cpuid(7);
        (cpuid.ebx & (1 << 5)) != 0
    }
}

#[cfg(target_arch = "x86_64")]
fn has_sse2_impl() -> bool {
    unsafe {
        let cpuid = std::arch::x86_64::__cpuid(1);
        (cpuid.edx & (1 << 26)) != 0
    }
}

#[cfg(target_arch = "x86_64")]
fn has_sse4_1_impl() -> bool {
    unsafe {
        let cpuid = std::arch::x86_64::__cpuid(1);
        (cpuid.ecx & (1 << 19)) != 0
    }
}

#[cfg(target_arch = "aarch64")]
fn has_neon_impl() -> bool {
    // ARM64 always has NEON support
    true
}

// Fallback for unsupported architectures
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn has_avx2_impl() -> bool { false }
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn has_sse2_impl() -> bool { false }
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn has_sse4_1_impl() -> bool { false }
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn has_neon_impl() -> bool { false }

// Convenience functions for checking specific features
pub fn has_avx2() -> bool {
    SimdCapabilities::detect().avx2
}

pub fn has_sse2() -> bool {
    SimdCapabilities::detect().sse2
}

pub fn has_sse4_1() -> bool {
    SimdCapabilities::detect().sse4_1
}

pub fn has_neon() -> bool {
    SimdCapabilities::detect().neon
}

pub fn get_simd_capabilities() -> SimdCapabilities {
    SimdCapabilities::detect()
}

pub fn get_best_simd_type() -> SimdType {
    SimdCapabilities::detect().get_best_simd_type()
}
