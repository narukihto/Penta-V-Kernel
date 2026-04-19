// src/shapes/triangle.rs

//! Implementation of the Triangle shape.
//! Poles: 3
//! Immunity Factor: 3 / 3 = 1.0 (Foundation Tier)

use super::GeometricBalancer;

pub struct Triangle;

impl GeometricBalancer for Triangle {
    /// Returns the number of poles for a triangle (3.0).
    fn poles(&self) -> f64 {
        3.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Triangle"
    }
}
