// src/utils/calculator.rs

//! # Geometric Decay Calculator
//! 
//! Optimized utility functions for translating external system stressors (deficits)
//! into internal kernel impact. This module integrates the geometric immunity 
//! factor with defensive Guard and Cooling protocols.

use crate::core::guard::Guard;
use crate::core::cooling::{CoolingProtocol, DECAY_COEFFICIENT};
use crate::core::KernelState;
use crate::shapes::GeometricBalancer;

/// Calculates and applies geometric decay to the kernel stability.
/// 
/// This function executes the core stability formula:
/// `Impact = (Deficit * DECAY_COEFFICIENT) / Immunity`
/// 
/// It utilizes **Static Dispatch** for zero-cost abstraction over geometric shapes
/// and ensures full integration with the system's defensive cooling layer.
#[inline(always)]
pub fn calculate_and_apply_decay<T: GeometricBalancer>(
    state: &mut KernelState,
    deficit: f64,
    shape: &T,
    cooling: &mut CoolingProtocol,
) {
    // 1. Input Sanitization: Ensure the stressor is a valid, positive magnitude.
    if !deficit.is_finite() || deficit <= 0.0 {
        return;
    }

    let immunity = shape.immunity_factor();

    // 2. Geometric Immunity Check: Handle asymptotic stability.
    // Infinite immunity (e.g., The Circle/Shield) nullifies all incoming stress.
    if immunity.is_infinite() || !immunity.is_finite() {
        return;
    }

    // 3. Impact Calculation: Determine the structural stability loss.
    let impact = (deficit * DECAY_COEFFICIENT) / immunity;

    // 4. Defensive Delegation: Execute the impact application via the Guard.
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
        
        let deficit = 10.0;
        calculate_and_apply_decay(&mut state, deficit, &triangle, &mut cooling);
        
        // Architecture Alignment:
        // The Guard applies a baseline protection layer (50% reduction) by default.
        // Formula: CORE_BASE - ((Deficit * DECAY) / Immunity * Guard_Mitigation)
        
        let immunity = triangle.immunity_factor();
        let raw_impact = (deficit * DECAY_COEFFICIENT) / immunity;
        let expected_stability = CORE_BASE - (raw_impact * 0.5);

        let diff = (state.current_stability - expected_stability).abs();
        
        assert!(
            diff < 1e-9, 
            "Stability deviation detected! Actual: {}, Expected: {}", 
            state.current_stability, 
            expected_stability
        );
    }

    #[test]
    fn test_nan_resilience() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        
        // Verify that invalid data (NaN) cannot breach kernel integrity.
        calculate_and_apply_decay(&mut state, f64::NAN, &Triangle, &mut cooling);
        assert_eq!(state.current_stability, CORE_BASE);
    }
}
