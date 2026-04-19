// src/shapes/mod.rs

//! Interface definitions for the geometric shapes supported by the protocol.
//! Each shape implements the GeometricBalancer trait to provide its immunity 
//! factor and handle load distribution logic.

pub mod triangle;
pub mod square;
pub mod pentagon;
pub mod hexagon;
pub mod heptagon;
pub mod octagon;
pub mod nonagon;
pub mod decagon;
pub mod dodecagon;
pub mod circle;

/// Trait defining the geometric stability capabilities of a shape.
/// Implementing this trait allows a shape to be used by the calculator 
/// and the kernel's defensive layer.
pub trait GeometricBalancer {
    /// Returns the number of poles (N) of the shape.
    /// This value is central to the load distribution calculation.
    fn poles(&self) -> f64;

    /// Calculates the immunity factor (N / 3.0).
    /// Higher values provide better resistance to external load.
    /// Default implementation uses static poles; can be overridden for custom shapes.
    #[inline(always)]
    fn immunity_factor(&self) -> f64 {
        let n = self.poles();
        // Handle the Circle (Infinity) case explicitly in the trait implementation
        if n.is_infinite() {
            f64::INFINITY
        } else {
            n / 3.0
        }
    }

    /// Returns the name of the geometric form for logging and audit purposes.
    fn name(&self) -> &str;
}
