// src/bridge/mod.rs

//! # Penta-Pack: Sovereign Bridge Module
//! 
//! This module orchestrates the packaging of Python logic into a standalone 
//! kernel binary, while managing geometric logic validation and legacy telemetry.

pub mod packer;
pub mod security;
pub mod validator;
pub mod telemetry;

pub use packer::SovereignPacker;
pub use security::StructuralGuard;
pub use validator::{LogicSignature, GeometricValidator};
pub use telemetry::HeartbeatMonitor;

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct BridgeConfig {
    #[pyo3(get, set)]
    pub self_contained: bool,
    #[pyo3(get, set)]
    pub security_tier: f64,
}

#[pymethods]
impl BridgeConfig {
    #[new]
    pub fn new(self_contained: bool, security_tier: f64) -> Self {
        Self { self_contained, security_tier }
    }
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            self_contained: true,
            security_tier: 0.8,
        }
    }
}

