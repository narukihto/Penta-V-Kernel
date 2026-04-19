// src/shapes/hexagon.rs

//! Implementation of the Hexagon shape.
//! Poles: 6
//! Immunity Factor: 2.0 (High structural stability)

use super::GeometricBalancer;

pub struct Hexagon;

impl GeometricBalancer for Hexagon {
    /// Returns the number of poles for a hexagon (6).
    #[inline(always)]
    fn poles(&self) -> f64 {
        6.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Hexagon"
    }
}
