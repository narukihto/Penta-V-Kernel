//! # Legacy Telemetry Monitor
//! 
//! Tracks performance stressors from legacy systems and triggers 
//! adaptive shape transitions to maintain kernel equilibrium.

use pyo3::prelude::*;
use std::time::Instant;

#[pyclass]
pub struct HeartbeatMonitor {
    last_pulse: Instant,
    pub debt_threshold: f64,
}

#[pymethods]
impl HeartbeatMonitor {
    #[new]
    pub fn new(debt_threshold: f64) -> Self {
        Self {
            last_pulse: Instant::now(),
            debt_threshold,
        }
    }

    /// Records a heartbeat and evaluates if technical debt stress necessitates a shape shift.
    /// Returns true if the stress-per-second exceeds the defined debt threshold.
    pub fn track_legacy_stress(&mut self, legacy_impact: f64) -> bool {
        let elapsed = self.last_pulse.elapsed().as_secs_f64().max(0.0001);
        self.last_pulse = Instant::now();

        // Formula: Legacy Debt Impact / Time Delta
        let stress_velocity = legacy_impact / elapsed;

        // Trigger adaptation if legacy stressors accelerate beyond the safety threshold
        stress_velocity > self.debt_threshold
    }
}
