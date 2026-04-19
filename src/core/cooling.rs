// src/core/cooling.rs

//! Thermal Management Unit for the Penta-V Kernel.
//! This module calculates the thermal dissipation efficiency 
//! based on the active geometric shape's immunity factor.

use crate::shapes::GeometricBalancer;

/// The base energy loss constant of the system.
pub const BASE_DECAY_COEFFICIENT: f64 = 0.02;

/// Thermal Controller that manages system "heat" (load impact).
pub struct ThermalController;

impl ThermalController {
    /// Calculates the effective decay coefficient.
    /// As the immunity factor increases, the thermal dissipation becomes more efficient.
    pub fn calculate_effective_decay(shape: &dyn GeometricBalancer) -> f64 {
        let immunity = shape.immunity_factor();
        
        if immunity.is_infinite() {
            // Perfect cooling state in Circle mode
            0.0 
        } else {
            // Higher immunity leads to lower effective decay (better cooling)
            BASE_DECAY_COEFFICIENT / immunity
        }
    }

    /// Determines the thermal stress level.
    pub fn get_stress_level(impact: f64) -> &'static str {
        if impact < 0.01 {
            "Optimal"
        } else if impact < 0.05 {
            "Nominal"
        } else {
            "Critical - Geometric Escalation Required"
        }
    }
}
