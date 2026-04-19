// src/core/cooling.rs

//! Thermal management for the Penta-V Kernel.

pub const DECAY_COEFFICIENT: f64 = 0.02;

pub enum CoolingState {
    Active,
    Idle,
}

pub struct CoolingProtocol {
    pub state: CoolingState,
}

impl CoolingProtocol {
    pub fn new() -> Self {
        Self {
            state: CoolingState::Active,
        }
    }

    pub fn apply_cooling(&self, impact: f64) -> f64 {
        impact * DECAY_COEFFICIENT
    }
}
