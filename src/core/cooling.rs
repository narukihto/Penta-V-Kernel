// src/core/cooling.rs

//! Thermal management for the Penta-V Kernel.

pub const DECAY_COEFFICIENT: f64 = 0.02;

#[derive(Debug, PartialEq)]
pub enum CoolingState {
    Active,
    Idle,
}

pub struct CoolingProtocol {
    pub state: CoolingState,
    /// The factor by which impact is reduced when cooling is active.
    pub reduction_factor: f64, 
}

impl CoolingProtocol {
    pub fn new() -> Self {
        Self {
            state: CoolingState::Idle,
            reduction_factor: 0.5, // تقليل الأثر بنسبة 50% عند التفعيل مثلاً
        }
    }

    /// Activates the cooling protocol.
    pub fn activate(&mut self) {
        self.state = CoolingState::Active;
    }

    pub fn apply_cooling(&self, impact: f64) -> f64 {
        if self.state == CoolingState::Active {
            impact * self.reduction_factor
        } else {
            impact
        }
    }
}
