// src/utils/calculator.rs

//! Optimized utility functions for precise stability calculations.
//! This module translates external load (deficit) into kernel impact,
//! integrating directly with the defensive Guard and Cooling protocols.

use crate::core::guard::Guard;
use crate::core::cooling::{CoolingProtocol, DECAY_COEFFICIENT};
use crate::core::KernelState; // Direct import from core for public visibility
use crate::shapes::GeometricBalancer;

/// Calculates the decay impact for a given deficit and applies it to the kernel state.
/// Uses static dispatch for maximum performance and integrates the cooling protocol
/// for reactive system protection.
#[inline(always)]
pub fn calculate_and_apply_decay<T: GeometricBalancer>(
    state: &mut KernelState,
    deficit: f64,
    shape: &T,
    cooling: &mut CoolingProtocol,
) {
    // 1. Input Sanitization: Ensure the deficit is a valid, positive number.
    if !deficit.is_finite() || deficit <= 0.0 {
        return;
    }

    let immunity = shape.immunity_factor();

    // 2. Structural Stability Check: Handle extreme geometric cases.
    // Infinite immunity (Circle) effectively nullifies impact.
    if immunity.is_infinite() {
        return;
    }

    // Sanitize immunity factor to prevent NaN propagation.
    if !immunity.is_finite() {
        return;
    }

    // 3. Impact Calculation: Determine the raw stability loss.
    // Formula: Impact = (Deficit * Decay) / Immunity
    let impact = (deficit * DECAY_COEFFICIENT) / immunity;

    // 4. Defensive Execution: Delegate to the Guard with cooling integration.
    if impact.is_finite() {
        Guard::apply_damage_with_cooling(state, impact, cooling);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::triangle::Triangle;
    use crate::core::{CORE_BASE, KernelState};
    use crate::core::cooling::CoolingProtocol;

    #[test]
    fn test_decay_calculation_logic() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        let triangle = Triangle;
        
        // Input: 10.0 units of deficit
        let deficit = 10.0;
        calculate_and_apply_decay(&mut state, deficit, &triangle, &mut cooling);
        
        // Manual Calculation for verification:
        // Raw Impact = (10.0 * 0.02) / 1.0 = 0.2
        // Since cooling is Idle by default, no reduction factor is applied.
        // Expected Stability = 1.0 - 0.2 = 0.8
        let raw_impact = (deficit * DECAY_COEFFICIENT) / triangle.immunity_factor();
        let expected = CORE_BASE - raw_impact;

        // Use a small epsilon for floating point comparison to avoid precision mismatch
        assert!((state.current_stability - expected).abs() < 1e-10);
    }

    #[test]
    fn test_nan_resilience() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        
        // Ensure invalid input (NaN) does not corrupt kernel stability
        calculate_and_apply_decay(&mut state, f64::NAN, &Triangle, &mut cooling);
        assert_eq!(state.current_stability, CORE_BASE);
    }
}
