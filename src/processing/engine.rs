// src/processing/engine.rs

//! # Processing Engine
//! 
//! Orchestrates the interaction between the PentaCleaner and the Kernel's 
//! CoolingProtocol to ensure data processing does not compromise system stability.

use crate::core::cooling::{CoolingProtocol, CoolingState};
use crate::processing::cleaner::PentaCleaner;
use crate::processing::ProcessingState;
use polars_core::prelude::DataFrame;

/// The orchestrator for thermal-aware data processing.
pub struct ProcessingEngine;

impl ProcessingEngine {
    /// Executes a cleaning task while adhering to kernel thermal constraints.
    /// 
    /// Logic:
    /// 1. Checks the current CoolingState from the core.
    /// 2. If the system is in 'Active' cooling, it throttles the processing intensity.
    /// 3. Applies geometric sanitization via PentaCleaner.
    pub fn dispatch_cleaning_task(
        df: &mut DataFrame,
        state: &mut ProcessingState,
        cooling: &CoolingProtocol,
    ) {
        // 1. Thermal Awareness: Check if the kernel is overheating
        match cooling.state {
            CoolingState::Active => {
                // Throttle: Reduce data pressure to protect the SECURE_CORE
                state.data_pressure *= 0.5;
                state.active_poles = 3; // Force down to Triangle (Foundation)
            }
            CoolingState::Idle => {
                // Full Performance: Allow high-intensity geometric poles
                state.active_poles = 12; // Dodecagon (Critical Stress Mode)
            }
        }

        // 2. Execution: Perform the actual sanitization
        let _ = PentaCleaner::geometric_sanitize(df, state);
    }
}
