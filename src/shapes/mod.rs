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
/// Penta-V load distribution protocol by defining its spatial pole configuration.
pub trait GeometricBalancer {
    /// Returns the number of geometric poles (N) of the shape.
    /// This value dictates the dissipation capacity of the structure.
    fn poles(&self) -> f64;

    /// Calculates the immunity factor (Φ) based on the N-pole configuration.
    /// 
    /// The default implementation uses the standard Φ = N / 3.0 formula,
    /// with explicit handling for asymptotic (infinite) stability.
    #[inline(always)]
    fn immunity_factor(&self) -> f64 {
        let n = self.poles();
        if n.is_infinite() {
            f64::INFINITY
        } else {
            n / 3.0
        }
    }

    /// Returns the canonical name of the geometric form.
    fn name(&self) -> &str;
}
