// src/core/cooling.rs

//! Thermal-aware resilience and entropy management for the Penta-V Kernel.
//! This module governs the dissipation of kinetic deficits during high-stress execution.

/// The rate at which system stressors are naturally dissipated across geometric poles.
pub const DECAY_COEFFICIENT: f64 = 0.02;

#[derive(Debug, PartialEq)]
/// Represents the operational state of the thermal mitigation substrate.
pub enum CoolingState {
    /// Active dissipation: The system is aggressively mitigating thermal jitter.
    Active,
    /// Idle: The system is operating within nominal geometric stability limits.
    Idle,
}

/// The Sovereign Cooling Protocol: Manages thermal-aware decay scaling.
/// As system heat increases, this protocol dynamically scales the stabilization functions.
pub struct CoolingProtocol {
    pub state: CoolingState,
    /// The scalar factor utilized to reduce kinetic impact when cooling is engaged.
    pub reduction_factor: f64, 
}

impl CoolingProtocol {
    /// Initializes a new cooling protocol with baseline reduction factors.
    pub fn new() -> Self {
        Self {
            state: CoolingState::Idle,
            reduction_factor: 0.5, // 50% impact reduction upon activation.
        }
    }

    /// Engages the cooling protocol, shifting the kernel into a high-resilience state.
    pub fn activate(&mut self) {
        self.state = CoolingState::Active;
    }

    /// Applies thermal mitigation to a calculated impact scalar.
    /// Used by the AI-Shield to anchor autonomous logic during stress surges.
    pub fn apply_cooling(&self, impact: f64) -> f64 {
        if self.state == CoolingState::Active {
            impact * self.reduction_factor
        } else {
            impact
        }
    }
}
