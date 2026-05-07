use crate::core::guard::Guard;
use crate::core::cooling::{CoolingProtocol, DECAY_COEFFICIENT};
use crate::core::KernelState;
use crate::shapes::GeometricShape;

#[inline(always)]
pub fn calculate_and_apply_decay<T: GeometricShape>(
    state: &mut KernelState,
    deficit: f64,
    shape: &T,
    cooling: &mut CoolingProtocol,
) {
    if !deficit.is_finite() || deficit <= 0.0 {
        return;
    }

    let immunity = shape.calculate_dissipation(state.current_stability);

    if immunity.is_infinite() || !immunity.is_finite() {
        return;
    }

    let impact = (deficit * DECAY_COEFFICIENT) / immunity;

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
        
        let immunity = triangle.calculate_dissipation(CORE_BASE);
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
        
        calculate_and_apply_decay(&mut state, f64::NAN, &Triangle, &mut cooling);
        assert_eq!(state.current_stability, CORE_BASE);
    }
}
