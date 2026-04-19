// src/core/mod.rs

//! Core module defining the fundamental constants, structures, and
//! protocols of the Geometric Stability Protocol.

pub mod guard;
pub mod cooling; 

/// The initial base stability of the kernel.
pub const CORE_BASE: f64 = 1.0;

/// The absolute minimum threshold that the core stability cannot fall below.
/// This acts as the strategic reserve to prevent total system collapse.
pub const SECURE_CORE: f64 = 0.05;

/// The pure decay coefficient used when calculating stability loss.
/// Any loss of one unit results in a deduction of 0.019 from the base.
pub const DECAY_COEFFICIENT: f64 = 0.019;

/// Represents the current stability state of the Penta-V Kernel.
#[derive(Debug, Clone, Copy)]
pub struct KernelState {
    pub current_stability: f64,
}

impl Default for KernelState {
    fn default() -> Self {
        Self {
            current_stability: CORE_BASE,
        }
    }
}

/// A centralized diagnostic check to verify if the kernel is currently 
/// operating in a critical state.
impl KernelState {
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        self.current_stability <= SECURE_CORE
    }
}
