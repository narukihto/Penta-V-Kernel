// src/shapes/triangle.rs

//! Implementation of the Triangle shape.
//! Poles: 3
//! Immunity Factor: 1.0 (Standard structural stability)

use super::GeometricBalancer;

pub struct Triangle;

impl GeometricBalancer for Triangle {
    /// Returns the number of poles (3).
    #[inline(always)]
    fn poles(&self) -> f64 {
        3.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Triangle"
    }
}
