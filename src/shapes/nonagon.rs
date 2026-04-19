// src/shapes/nonagon.rs

//! Implementation of the Nonagon shape.
//! Poles: 9
//! Immunity Factor: 9 / 3 = 3.0 (Triple Stability Milestone)

use super::GeometricBalancer;

pub struct Nonagon;

impl GeometricBalancer for Nonagon {
    /// Returns the number of poles for a nonagon (9.0).
    fn poles(&self) -> f64 {
        9.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Nonagon"
    }
}
