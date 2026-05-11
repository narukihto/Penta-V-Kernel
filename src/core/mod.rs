// src/core/mod.rs

//! # Core Module
//!
//! Defines the fundamental constants, structures, and protocols of the 
//! Geometric Stability Protocol. This module acts as the "Heart" of the Penta-V Kernel.

pub mod guard;
pub mod cooling; 

// --- Re-exports for Sovereign Accessibility ---
// This allows external crates and benchmarks to use:
// `penta_v_kernel::core::Guard` instead of `penta_v_kernel::core::guard::Guard`
pub use guard::Guard;
pub use cooling::CoolingProtocol;

/// The initial base stability of the kernel (1.0).
pub const CORE_BASE: f64 = 1.0;

/// The absolute minimum threshold that the core stability cannot fall below (0.05).
/// This acts as the strategic reserve to prevent total system collapse.
pub const SECURE_CORE: f64 = 0.05;

/// The pure decay coefficient used when calculating stability loss (0.019).
/// Any loss of one unit results in a deduction of 0.019 from the base.
pub const DECAY_COEFFICIENT: f64 = 0.019;

/// Represents the current stability state of the Penta-V Kernel.
#[derive(Debug, Clone, Copy)]
pub struct KernelState {
    /// The current normalized stability value (0.0 to 1.0).
    pub current_stability: f64,
}

impl Default for KernelState {
    fn default() -> Self {
        Self {
            current_stability: CORE_BASE,
        }
    }
}

impl KernelState {
    /// Initializes a new KernelState at the unitary stability baseline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the kernel stability is at or below the SECURE_CORE threshold.
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        self.current_stability <= SECURE_CORE
    }
}
