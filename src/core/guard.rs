// src/core/guard.rs

//! The 'Guard' module acts as the primary defensive layer for the kernel.
//! It enforces the SECURE_CORE constraint and manages the cooling protocol
//! to ensure system stability under extreme load.

use super::{SECURE_CORE, KernelState};
use crate::core::cooling::CoolingProtocol;

pub struct Guard;

impl Guard {
    /// Applies impact to kernel stability.
    /// If stability drops below the SECURE_CORE threshold, it automatically
    /// triggers the CoolingProtocol to protect the system integrity.
    ///
    /// # Arguments
    /// * `state` - The current KernelState to modify.
    /// * `impact` - The calculated stability loss from the load balancer.
    /// * `cooling` - The CoolingProtocol instance to manage throttle states.
    #[inline(always)]
    pub fn apply_damage_with_cooling(
        state: &mut KernelState,
        impact: f64,
        cooling: &mut CoolingProtocol,
    ) {
        // 1. Input Sanitization: Validate impact to prevent undefined behavior.
        if !impact.is_finite() || impact < 0.0 {
            return;
        }

        // 2. Cooling Logic: If the system is in cooling mode, reduce the effective impact.
        let effective_impact = impact * cooling.reduction_factor;
        let new_stability = state.current_stability - effective_impact;

        // 3. Threshold Enforcement: Ensure the system never collapses.
        if new_stability <= SECURE_CORE {
            state.current_stability = SECURE_CORE;
            
            // Trigger the cooling protocol immediately when the threshold is hit.
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
    use crate::core::cooling::CoolingProtocol;

    #[test]
    fn test_guard_enforcement_and_cooling() {
        let mut state = KernelState { current_stability: CORE_BASE };
        let mut cooling = CoolingProtocol::new();
        
        // Simulate a critical load event that forces the system into cooling mode.
        Guard::apply_damage_with_cooling(&mut state, 9999.0, &mut cooling);
        
        // Assert protection
        assert_eq!(state.current_stability, SECURE_CORE);
        // Assert cooling activation
        assert!(matches!(cooling.state, crate::core::cooling::CoolingState::Active));
    }
}
