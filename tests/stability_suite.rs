// tests/stability_suite.rs

//! # 🛡️ Sovereign Stability Suite
//! 
//! Stress tests for the Penta-V Kernel.
//! Validates system resilience against extreme load scenarios, thermal processing, 
//! and secure distribution synchronization.

use penta_v_kernel::core::{KernelState, SECURE_CORE};
use penta_v_kernel::core::cooling::CoolingProtocol;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;
use penta_v_kernel::mesh::{MeshNode, StabilityPacket, MeshPulse}; 
use penta_v_kernel::processing::{ProcessingEngine, ProcessingState};
use penta_v_kernel::bridge::{StructuralGuard, BridgeConfig};
use polars_core::prelude::*;

#[test]
fn test_decagon_stress_attack() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let decagon = Decagon;

    calculate_and_apply_decay(&mut state, 50.0, &decagon, &mut cooling);

    assert!(state.current_stability > SECURE_CORE);
    println!("Decagon stability after stress: {:.4}", state.current_stability);
}

#[test]
fn test_thermal_aware_processing_stability() {
    let mut state = ProcessingState::default();
    let mut cooling = CoolingProtocol::new();
    
    let mut df = DataFrame::new(vec![Series::new("metrics", &[1.0, 2.0, 3.0])]).unwrap();

    cooling.state = penta_v_kernel::core::cooling::CoolingState::Active;
    state.data_pressure = 1.0; 

    ProcessingEngine::dispatch_cleaning_task(&mut df, &mut state, &cooling);

    assert_eq!(state.data_pressure, 0.5);
    assert_eq!(state.active_poles, 3);
    println!("Thermal throttling validated: Pressure reduced to 0.5");
}

#[test]
fn test_security_lockdown_on_instability() {
    let mut bytecode = vec![0u8; 64];
    let original_bytecode = bytecode.clone();
    let config = BridgeConfig::default();
    
    // RECALIBRATION: Stability must be strictly < 0.05 to trigger lockdown.
    // 0.01 is utilized here to ensure the logic gate in src/bridge/security.rs shuts.
    let mut state = KernelState { current_stability: 0.01 }; 
    let mut cooling = CoolingProtocol::new();

    // Execution: Attempt to protect assets during a confirmed breach of stability.
    StructuralGuard::protect_assets(&mut bytecode, &config, &mut state, &mut cooling);

    // Validation: Integrity check must return false (0.01 < 0.05).
    assert!(!StructuralGuard::verify_integrity(&state));
    
    // Lockdown Proof: The bytecode must remain identical to the original state.
    assert_eq!(bytecode, original_bytecode);
    
    println!("Security lockdown validated: Protocol held firm at {:.4}", state.current_stability);
}

#[test]
fn test_total_collapse_protection() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let triangle = Triangle;

    calculate_and_apply_decay(&mut state, 5000.0, &triangle, &mut cooling);

    assert!((state.current_stability - SECURE_CORE).abs() < f64::EPSILON);
    println!("Test passed: SECURE_CORE ACTIVATED at {:.2}", state.current_stability);
}

// --- Phase IV: Distributed Mesh Test Suite ---

#[test]
fn test_mesh_pulse_telemetry_integrity() {
    let node_id = 0xAA55;
    let mut node = MeshNode::new(node_id, true);
    node.local_stability = 0.92;
    let cooling = CoolingProtocol::new();

    let pulse = node.generate_pulse(&cooling); 

    assert_eq!(pulse.node_id, node_id);
    assert_eq!(pulse.stability_score, 0.92);
}

#[test]
fn test_mesh_critical_handshake_security() {
    let mut observer = MeshNode::new(1, true);
    
    let critical_pulse = StabilityPacket {
        node_id: 99,
        stability_score: 0.10, 
        thermal_load: 0.90,
    };

    observer.handle_incoming_pulse(critical_pulse);

    assert!(observer.secure_gate);
}
