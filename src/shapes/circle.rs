// src/shapes/circle.rs

//! # Circle Geometry (The Asymptotic Shield)
//! 
//! Implementation of the Circle shape which represents the theoretical limit 
//! of geometric stability. By utilizing an infinite number of poles, 
//! it achieves total dissipation of system stressors.

use crate::shapes::GeometricBalancer;

/// Represents the Circle geometric configuration.
/// 
/// In the Penta-V protocol, the Circle acts as the ultimate defensive barrier,
/// providing infinite immunity ($\Phi = \infty$) by distributing load across
/// an asymptotic boundary.
pub struct Circle;

impl GeometricBalancer for Circle {
    /// Returns infinity as the number of poles.
    /// This ensures that the dissipation factor reaches its theoretical maximum.
    #[inline(always)]
    fn poles(&self) -> f64 { 
        f64::INFINITY 
    }

    /// Returns the canonical name for the asymptotic shield.
    #[inline(always)]
    fn name(&self) -> &'static str { 
        "Circle" 
    }
}
