// src/shapes/pentagon.rs

//! Implementation of the Pentagon shape.
//! Poles: 5
//! Immunity Factor: 1.66 (Intermediate structural stability)

use super::GeometricBalancer;

pub struct Pentagon;

impl GeometricBalancer for Pentagon {
    /// Returns the number of poles for a pentagon (5).
    #[inline(always)]
    fn poles(&self) -> f64 {
        5.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Pentagon"
    }
}
