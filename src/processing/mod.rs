// src/processing/mod.rs

//! # Penta-Clean: Processing Module
//! 
//! This module handles high-performance data sanitization and pattern discovery 
//! by leveraging the kernel's geometric dissipation logic.

pub mod cleaner;
pub mod engine;

pub use cleaner::PentaCleaner;
pub use engine::ProcessingEngine;

/// Internal state for data pressure monitoring
pub struct ProcessingState {
    /// Density of the current data packet
    pub data_pressure: f64,
    /// Number of active geometric poles assigned to cleaning
    pub active_poles: u32,
}

impl Default for ProcessingState {
    fn default() -> Self {
        Self {
            data_pressure: 0.0,
            active_poles: 3, // Defaults to Triangle (Foundation)
        }
    }
}
