//! # Geometric Logic Validator
//! 
//! Validates performance metrics from Python/AI-generated code against 
//! the current Geometric Poles to prevent logic-induced kernel instability.

use pyo3::prelude::*;
use crate::core::{KernelState, SECURE_CORE};
use crate::shapes::GeometricShape;

#[pyclass]
#[derive(Clone)]
pub struct LogicSignature {
    pub stress_level: f64,
    pub complexity_index: f64,
}

#[pymethods]
impl LogicSignature {
    #[new]
    pub fn new(stress_level: f64, complexity_index: f64) -> Self {
        Self { stress_level, complexity_index }
    }
}

pub struct GeometricValidator;

impl GeometricValidator {
    /// Validates if the external code's stress signature is compatible 
    /// with the current active geometric shape's dissipation capacity.
    pub fn validate_logic_stability(
        state: &KernelState,
        signature: &LogicSignature,
        active_shape: &dyn GeometricShape
    ) -> bool {
        // Step 1: Calculate the dissipation ceiling for the current stability level.
        let capacity = active_shape.calculate_dissipation(state.current_stability);
        
        // Step 2: Evaluate the logic's impact (Stress amplified by Complexity).
        let logical_impact = signature.stress_level * (1.0 + signature.complexity_index);

        // Step 3: Sovereign Validation Gate.
        // Logic is stable ONLY if it remains within capacity AND strictly above SECURE_CORE.
        logical_impact <= capacity && state.current_stability > SECURE_CORE
    }
}
