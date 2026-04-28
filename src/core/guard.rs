// src/core/guard.rs

//! # The Guard Layer
//! 
//! The Guard acts as the final arbiter of kernel stability. It enforces the 
//! `SECURE_CORE` constraint—a mathematical boundary that prevents system collapse.
//! It also orchestrates the transition into cooling states during thermal surges.

use super::{SECURE_CORE, KernelState};
use crate::core::cooling::CoolingProtocol;

/// The primary defensive sentinel for the Penta-V Kernel.
pub struct Guard;

impl Guard {
    /// Mediates and applies calculated stressors to the kernel's stability state.
    /// 
    /// This function implements the 'Defensive Throttle' logic:
    /// 1. Sanitizes input to prevent NaN or infinite dissipation errors.
    /// 2. Applies the thermal reduction factor if the `CoolingProtocol` is active.
    /// 3. Clamps the final stability to the `SECURE_CORE` threshold.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the current [KernelState].
    /// * `impact` - Raw stability loss calculated by the geometric balancer.
    /// * `cooling` - Mutable reference to the [CoolingProtocol] for throttle management.
    #[inline(always)]
    pub fn apply_damage_with_cooling(
        state: &mut KernelState,
        impact: f64,
        cooling: &mut CoolingProtocol,
    ) {
        // 1. Input Sanitization: Critical for preventing 'Silent Corruption' in systems.
        if !impact.is_finite() || impact < 0.0 {
            return;
        }

        // 2. Thermal Mitigation: Apply the cooling reduction factor.
        let effective_impact = impact * cooling.reduction_factor;
        let new_stability = state.current_stability - effective_impact;

        // 3. Absolute Boundary Enforcement: The SECURE_CORE is non-negotiable.
        if new_stability <= SECURE_CORE {
            state.current_stability = SECURE_CORE;
            
            // Immediate escalation to active cooling upon threshold breach.
            cooling.activate();
        } else {
            state.current_stability = new_stability;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{CORE_BASE, KernelState};
    use crate::core::cooling::{CoolingProtocol, CoolingState};

    #[test]
    fn test_guard_enforcement_and_cooling() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        
        // Critical Scenario: An overwhelming deficit of 9999.0 units.
        Guard::apply_damage_with_cooling(&mut state, 9999.0, &mut cooling);
        
        // Assert: Stability must be clamped to the strategic reserve.
        assert_eq!(state.current_stability, SECURE_CORE);
        
        // Assert: Protocol escalation must be triggered.
        assert!(matches!(cooling.state, CoolingState::Active));
    }
}
