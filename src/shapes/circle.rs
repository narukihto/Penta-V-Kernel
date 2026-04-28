// src/shapes/circle.rs

use super::GeometricBalancer;

/// Represents a Circle geometry (Asymptotic Defense).
pub struct Circle;

impl GeometricBalancer for Circle {
    /// Returns infinity-like poles for a circle.
    #[inline(always)]
    fn poles(&self) -> f64 {
        f64::INFINITY
    }

    fn name(&self) -> &'static str {
        "Circle"
    }
}
