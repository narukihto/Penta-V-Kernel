// src/shapes/mod.rs

//! Interface definitions for the geometric shapes supported by the protocol.
//! Each shape implements the GeometricShape trait to provide its dissipation 
//! capacity and handle load distribution logic.

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

// Re-export shapes for direct access via the shapes module
pub use triangle::Triangle;
pub use square::Square;
pub use pentagon::Pentagon;
pub use hexagon::Hexagon;
pub use heptagon::Heptagon;
pub use octagon::Octagon;
pub use nonagon::Nonagon;
pub use decagon::Decagon;
pub use dodecagon::Dodecagon;
pub use circle::Circle;

/// Trait defining the geometric stability capabilities of a shape.
/// 
/// Implementing this trait allows a structure to participate in the 
/// Penta-V load distribution protocol and logic validation.
pub trait GeometricShape: Send + Sync {
    /// Returns the number of geometric poles (N) of the shape.
    fn poles(&self) -> f64;

    /// Calculates the dissipation capacity (formerly immunity factor) 
    /// based on the N-pole configuration and current stability.
    #[inline(always)]
    fn calculate_dissipation(&self, _stability: f64) -> f64 {
        let n = self.poles();
        if n.is_infinite() {
            f64::INFINITY
        } else {
            // Standard Dissipation Formula: Φ = N / 3.0
            n / 3.0
        }
    }

    /// Returns the canonical name of the geometric form.
    fn name(&self) -> &str;
}

/// Legacy Alias: Keeps backward compatibility if other modules use the old name.
pub use GeometricShape as GeometricBalancer;
