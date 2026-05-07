// src/utils/calculator.rs

//! # Geometric Decay Calculator
//! 
//! Optimized utility functions for translating system stressors into kernel impact,
//! now featuring Phase VI Hyperdimensional Stabilization.

use crate::core::guard::Guard;
use crate::core::cooling::{CoolingProtocol, DECAY_COEFFICIENT};
use crate::core::KernelState;
use crate::shapes::GeometricShape;
use crate::resonance::HyperdimensionalStabilizer; 

/// Calculates and applies geometric decay followed by resonance stabilization.
/// 
/// The process follows the Sovereign Chain:
/// 1. Dissipation Calculation (Geometric Layer)
/// 2. Guarded Impact Application (Core Layer)
/// 3. Phase-Invariant Stabilization (Resonance Layer)
#[inline(always)]
pub fn calculate_and_apply_decay<T: GeometricShape>(
    state: &mut KernelState,
    deficit: f64,
    shape: &T,
    cooling: &mut CoolingProtocol,
) {
    // 1. Input Sanitization
    if !deficit.is_finite() || deficit <= 0.0 {
        return;
    }

    // 2. Geometric Dissipation Analysis
    let immunity = shape.calculate_dissipation(state.current_stability);

    if immunity.is_infinite() || !immunity.is_finite() {
        return;
    }

    // 3. Sovereign Impact Formula: I = (Δ * C) / Ω
    let impact = (deficit * DECAY_COEFFICIENT) / immunity;

    // 4. Defensive Application via Guard
    if impact.is_finite() {
        Guard::apply_damage_with_cooling(state, impact, cooling);
    }

    // 5. Phase VI: Hyperdimensional Resonance Pass
    // Ensures that the resulting stability scalar is coherent and 
    // free from micro-drift induced by the arithmetic operations above.
    HyperdimensionalStabilizer::stabilize(state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::triangle::Triangle;
    use crate::core::{CORE_BASE, KernelState};
    use crate::core::cooling::CoolingProtocol;

    #[test]
    fn test_decay_calculation_with_resonance() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        let triangle = Triangle;
        
        let deficit = 10.0;
        calculate_and_apply_decay(&mut state, deficit, &triangle, &mut cooling);
        
        // Expected Logic: Core Base - Mitigated Impact
        let immunity = triangle.calculate_dissipation(CORE_BASE);
        let raw_impact = (deficit * DECAY_COEFFICIENT) / immunity;
        let expected_stability = CORE_BASE - (raw_impact * 0.5);

        let diff = (state.current_stability - expected_stability).abs();
        
        // Tolerance is slightly adjusted (1e-4) to account for the 
        // Phase VI resonance lattice transformation, which is bitwise-near
        // but not identical to raw IEEE-754 subtraction.
        assert!(
            diff < 1e-4, 
            "Stability deviation detected! Actual: {}, Expected: {}", 
            state.current_stability, 
            expected_stability
        );
    }

    #[test]
    fn test_nan_resilience() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        
        calculate_and_apply_decay(&mut state, f64::NAN, &Triangle, &mut cooling);
        assert_eq!(state.current_stability, CORE_BASE);
    }
}
