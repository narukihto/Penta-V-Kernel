// tests/stability_suite.rs

//! Stress tests for the Penta-V Kernel.
//! Validates system resilience against extreme load scenarios and mesh synchronization.

use penta_v_kernel::core::{KernelState, SECURE_CORE};
use penta_v_kernel::core::cooling::CoolingProtocol;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;
use penta_v_kernel::mesh::{MeshNode, MeshPulse, StabilityPacket};

#[test]
fn test_decagon_stress_attack() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let decagon = Decagon;

    // Simulate 50 units of deficit dissipation through Decagon geometry
    calculate_and_apply_decay(&mut state, 50.0, &decagon, &mut cooling);

    assert!(state.current_stability > SECURE_CORE);
    println!("Decagon stability after stress: {:.4}", state.current_stability);
}

#[test]
fn test_total_collapse_protection() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let triangle = Triangle;

    // Simulate 5000 units of deficit to trigger the SECURE_CORE Guard layer
    calculate_and_apply_decay(&mut state, 5000.0, &triangle, &mut cooling);

    // Verify protection layer activation (held at SECURE_CORE threshold).
    // Tolerance accommodates the Phase VI Hyperdimensional Resonance
    // Lattice traversal, which introduces bounded IEEE-754 phase drift.
    assert!((state.current_stability - SECURE_CORE).abs() < 1e-4);
    println!("Test passed: SECURE_CORE ACTIVATED at {:.2}", state.current_stability);
}

// --- Phase IV: Distributed Mesh Test Suite ---

#[test]
fn test_mesh_pulse_telemetry_integrity() {
    // Context: Initialize a kernel node with a specific stability state
    let node_id = 0xAA55;
    let mut node = MeshNode::new(node_id, true);
    node.local_stability = 0.92;

    // Execution: Generate a stability telemetry packet (Pulse)
    let pulse = node.generate_pulse();

    // Validation: Ensure pulse telemetry accurately reflects the source kernel state
    assert_eq!(pulse.node_id, node_id);
    assert_eq!(pulse.stability_score, 0.92);
}

#[test]
fn test_mesh_critical_handshake_security() {
    // Context: Setup a secure observer node within the mesh
    let mut observer = MeshNode::new(1, true);
    
    // Simulate a critical heartbeat from a remote failing peer
    let critical_pulse = StabilityPacket {
        node_id: 99,
        stability_score: 0.10, // Stability dropped below SECURE_CORE/Critical threshold
        thermal_load: 0.90,
    };

    // Execution: Observer node processes the remote failure signal
    observer.handle_incoming_pulse(critical_pulse);

    // Validation: Verify the observer maintains its security gate during processing
    assert!(observer.secure_gate);
}
