// src/shapes/heptagon.rs

//! Implementation of the Heptagon shape.
//! Poles: 7
//! Immunity Factor: 2.33 (Enhanced structural stability)

use super::GeometricBalancer;

pub struct Heptagon;

impl GeometricBalancer for Heptagon {
    /// Returns the number of poles for a heptagon (7).
    #[inline(always)]
    fn poles(&self) -> f64 {
        7.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Heptagon"
    }
}
