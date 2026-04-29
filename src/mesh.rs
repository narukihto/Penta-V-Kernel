// src/mesh.rs

//! # Mesh Network Protocol
//! 
//! Handles inter-kernel stability state synchronization and distributed load sharing.

/// Represents the systemic state transmitted across mesh nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StabilityPacket {
    /// Unique identifier for the sending kernel instance.
    pub node_id: u64,           
    /// Current stability score (0.0 to 1.0).
    pub stability_score: f64,   
    /// Real-time thermal-aware load state.
    pub thermal_load: f64,      
}

/// Interface for generating and processing mesh heartbeats.
pub trait MeshPulse {
    /// Generates a new stability packet based on current local metrics.
    fn generate_pulse(&self) -> StabilityPacket;
    
    /// Processes an incoming packet and adjusts local dissipation strategy.
    fn handle_incoming_pulse(&mut self, packet: StabilityPacket);
}

/// Logical representation of a participant within the geometric mesh.
pub struct MeshNode {
    pub id: u64,
    /// Protocol trust boundary flag.
    pub secure_gate: bool, 
    /// Current internal stability cached for the mesh.
    pub local_stability: f64,
}

impl MeshNode {
    /// Constructor for a new MeshNode instance.
    pub fn new(id: u64, secure_gate: bool) -> Self {
        Self { 
            id, 
            secure_gate,
            local_stability: 1.0, 
        }
    }
}

impl MeshPulse for MeshNode {
    fn generate_pulse(&self) -> StabilityPacket {
        StabilityPacket {
            node_id: self.id,
            stability_score: self.local_stability,
            thermal_load: 1.0 - self.local_stability, // Simplified thermal derivation
        }
    }

    fn handle_incoming_pulse(&mut self, packet: StabilityPacket) {
        if self.secure_gate && packet.stability_score < 0.2 {
            // Logic: If a peer is failing (< 0.2), prepare for load absorption.
            // This is where Phase IV redirection logic will be implemented.
        }
    }
}
