// src/shapes/square.rs

//! Implementation of the Square shape.
//! Poles: 4
//! Immunity Factor: 1.33 (Balanced structural stability)

use super::GeometricBalancer;

pub struct Square;

impl GeometricBalancer for Square {
    /// Returns the number of poles for a square (4).
    #[inline(always)]
    fn poles(&self) -> f64 {
        4.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Square"
    }
}
