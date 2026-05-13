use crate::core::cooling::{CoolingProtocol, CoolingState};
use crate::core::KernelState;
use crate::processing::cleaner::PentaCleaner;
use crate::processing::ProcessingState;
use crate::bridge::validator::{LogicSignature, GeometricValidator};
use crate::shapes::GeometricShape;
use polars_core::prelude::DataFrame;

pub struct ProcessingEngine;

impl ProcessingEngine {
    /// Dispatches a geometric cleaning task verified by the Sovereign Logic Signature.
    pub fn dispatch_cleaning_task(
        df: &mut DataFrame,
        state: &mut ProcessingState,
        kernel_state: &KernelState,
        cooling: &CoolingProtocol,
        signature: &LogicSignature,
        active_shape: &dyn GeometricShape,
    ) -> Result<(), String> {
        
        // Phase VI Logic Validation: Ensuring logic signatures don't drift into instability.
        if !GeometricValidator::validate_logic_stability(kernel_state, signature, active_shape) {
            return Err("Logic Validation Failed: Stress signature exceeds geometric manifold capacity.".into());
        }

        // Thermal-Aware State Management: Adjusting geometric poles based on system entropy.
        match cooling.state {
            CoolingState::Active => {
                state.data_pressure *= 0.5; // Dissipating kinetic deficit.
                state.active_poles = 3;     // Resetting to foundational stability (Triangle).
            }
            CoolingState::Idle => {
                state.active_poles = 12;    // High-fidelity stabilization (Dodecagon).
            }
        }

        // Executive sanitization pass through the PentaCleaner substrate.
        let _ = PentaCleaner::geometric_sanitize(df, state);
        
        Ok(())
    }
}
