//! # 🛡️ Sovereign Integration Suite
//! 
//! Critical integration tests validating the synergy between Geometric Validation,
//! Legacy Telemetry, and the Structural Lockdown protocol.

use penta_v_kernel::core::{KernelState, SECURE_CORE};
use penta_v_kernel::core::cooling::CoolingProtocol;
use penta_v_kernel::bridge::{
    LogicSignature, 
    GeometricValidator, 
    HeartbeatMonitor, 
    StructuralGuard, 
    BridgeConfig
};
use penta_v_kernel::shapes::triangle::Triangle;
use std::thread;
use std::time::Duration;

#[test]
fn test_ai_logic_rejection_and_lockdown_synergy() {
    // 1. Setup Environment
    let mut bytecode = vec![0u8; 32];
    let original_bytecode = bytecode.clone();
    let config = BridgeConfig::default();
    let mut cooling = CoolingProtocol::new();
    let mut state = KernelState { current_stability: 0.06 }; // Near-critical but stable
    let shape = Triangle;

    // 2. Scenario: AI generates a high-stress, high-complexity logic signature
    // This signature is designed to exceed the Triangle shape's dissipation capacity.
    let bad_ai_logic = LogicSignature::new(0.8, 0.9); 

    // 3. Step A: Validator must intercept the "bad" logic
    let is_valid = GeometricValidator::validate_logic_stability(&state, &bad_ai_logic, &shape);
    assert!(!is_valid, "Sovereign Error: Validator should have REJECTED the high-stress AI logic");

    // 4. Step B: Security Guard must maintain Lockdown because validator failed
    // We simulate a call to protect_assets under these unstable conditions
    if !is_valid {
        StructuralGuard::protect_assets(&mut bytecode, &config, &mut state, &mut cooling);
    }

    // 5. Validation: Bytecode must remain UNTOUCHED (Lockdown held firm)
    assert_eq!(bytecode, original_bytecode, "Lockdown Breach: Bytecode modified despite logic validation failure");
    println!("Synergy Verified: Validator rejected logic and Guard enforced Lockdown.");
}

#[test]
fn test_legacy_telemetry_adaptation_trigger() {
    // 1. Setup Telemetry with a debt threshold of 0.5 impact/sec
    let mut monitor = HeartbeatMonitor::new(0.5);
    
    // Simulate initial pulse
    monitor.track_legacy_stress(0.1);
    
    // 2. Wait for a small delta to simulate time passage
    thread::sleep(Duration::from_millis(100));

    // 3. Scenario: Legacy code spikes with high impact (5.0 units)
    // This should trigger the 'True' response for adaptation.
    let needs_adaptation = monitor.track_legacy_stress(5.0);

    assert!(needs_adaptation, "Telemetry Failure: System failed to detect legacy stress acceleration");
    println!("Telemetry Verified: Legacy stress velocity detected successfully.");
}

#[test]
fn test_critical_threshold_boundary_integrity() {
    // Ensuring the Validator respects the EXACT SECURE_CORE boundary (0.05)
    let state_at_boundary = KernelState { current_stability: SECURE_CORE };
    let safe_logic = LogicSignature::new(0.01, 0.01);
    let shape = Triangle;

    // Even "safe" logic must be rejected if we are AT the boundary (Lockdown Zone)
    let is_valid = GeometricValidator::validate_logic_stability(&state_at_boundary, &safe_logic, &shape);
    
    assert!(!is_valid, "Boundary Breach: Validator allowed logic execution at SECURE_CORE threshold");
    println!("Boundary Integrity: Validator correctly enforced Lockdown at {:.4}", SECURE_CORE);
}
