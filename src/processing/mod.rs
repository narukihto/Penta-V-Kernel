pub mod cleaner;
pub mod engine;

pub use cleaner::PentaCleaner;
pub use engine::ProcessingEngine;

pub struct ProcessingState {
    pub data_pressure: f64,
    pub active_poles: u32,
}

impl Default for ProcessingState {
    fn default() -> Self {
        Self {
            data_pressure: 0.0,
            active_poles: 3, 
        }
    }
}
