// src/mesh.rs

//! # Mesh Network Protocol
//! 
//! Handles inter-kernel stability state synchronization and distributed load sharing logic.

/// Represents a systemic state snapshot for mesh-wide synchronization.
#[derive(Debug, Clone, Copy)]
pub struct StabilityPacket {
    /// Unique identifier for the source kernel instance.
    pub node_id: u64,           
    /// Current stability score (0.0 to 1.0).
    pub stability_score: f64,   
    /// Current thermal-aware load percentage.
    pub thermal_load: f64,      
}

/// Logical representation of a participant within the geometric mesh.
pub struct MeshNode {
    pub id: u64,
    /// Administrative gate to enforce protocol trust boundaries.
    pub secure_gate: bool, 
}

impl MeshNode {
    /// New instance constructor for MeshNode.
    pub fn new(id: u64, secure_gate: bool) -> Self {
        Self { id, secure_gate }
    }

    /// Evaluates if the local node can absorb overflow deficits from the network.
    /// Returns true if stability is high (>0.8) and thermal load is low (<0.4).
    pub fn can_assist(&self, packet: &StabilityPacket) -> bool {
        self.secure_gate && packet.stability_score > 0.8 && packet.thermal_load < 0.4
    }
}
