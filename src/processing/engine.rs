//! # Processing Engine
//! 
//! Orchestrates the interaction between the PentaCleaner, the Kernel's 
//! CoolingProtocol, and the GeometricValidator to ensure data processing 
//! adheres to both thermal constraints and logic stability signatures.

use crate::core::cooling::{CoolingProtocol, CoolingState};
use crate::processing::cleaner::PentaCleaner;
use crate::processing::ProcessingState;
use crate::bridge::validator::{LogicSignature, GeometricValidator};
use crate::shapes::GeometricShape;
use polars_core::prelude::DataFrame;

/// The orchestrator for thermal-aware and logic-validated data processing.
pub struct ProcessingEngine;

impl ProcessingEngine {
    /// Executes a cleaning task while adhering to kernel thermal constraints 
    /// and pre-validating the logic signature.
    /// 
    /// Updated Logic (Sovereign Integration):
    /// 1. Pre-validates the AI logic signature against the active shape and kernel state.
    /// 2. Checks the current CoolingState from the core.
    /// 3. Throttles processing intensity if cooling is active.
    /// 4. Applies geometric sanitization via PentaCleaner.
    pub fn dispatch_cleaning_task(
        df: &mut DataFrame,
        state: &mut ProcessingState,
        cooling: &CoolingProtocol,
        signature: &LogicSignature,
        active_shape: &dyn GeometricShape,
    ) -> Result<(), String> {
        // --- STEP 1: LOGIC VALIDATION (Sovereign Listening) ---
        // We consult the Validator before any resource allocation or execution.
        if !GeometricValidator::validate_logic_stability(&state.kernel, signature, active_shape) {
            return Err("Logic Validation Failure: Code stress signature exceeds shape capacity.".into());
        }

        // --- STEP 2: THERMAL AWARENESS ---
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

        // --- STEP 3: EXECUTION ---
        let _ = PentaCleaner::geometric_sanitize(df, state);
        
        Ok(())
    }
}
