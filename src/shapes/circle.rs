// src/shapes/circle.rs
use crate::shapes::GeometricBalancer;

/// Represents a Circle geometry.
pub struct Circle;

impl GeometricBalancer for Circle {
    fn poles(&self) -> f64 { f64::INFINITY }
    fn name(&self) -> &'static str { "Circle" }
}
