use crate::core::cooling::{CoolingProtocol, CoolingState};
use crate::core::KernelState;
use crate::processing::cleaner::PentaCleaner;
use crate::processing::ProcessingState;
use crate::bridge::validator::{LogicSignature, GeometricValidator};
use crate::shapes::GeometricShape;
use polars_core::prelude::DataFrame;

pub struct ProcessingEngine;

impl ProcessingEngine {
    pub fn dispatch_cleaning_task(
        df: &mut DataFrame,
        state: &mut ProcessingState,
        kernel_state: &KernelState,
        cooling: &CoolingProtocol,
        signature: &LogicSignature,
        active_shape: &dyn GeometricShape,
    ) -> Result<(), String> {
        
        if !GeometricValidator::validate_logic_stability(kernel_state, signature, active_shape) {
            return Err("فشل التحقق المنطقي: توقيع الإجهاد يتجاوز سعة الشكل الهندسي".into());
        }

        match cooling.state {
            CoolingState::Active => {
                state.data_pressure *= 0.5;
                state.active_poles = 3; 
            }
            CoolingState::Idle => {
                state.active_poles = 12; 
            }
        }

        let _ = PentaCleaner::geometric_sanitize(df, state);
        
        Ok(())
    }
}
