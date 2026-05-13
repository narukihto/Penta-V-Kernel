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
    let mut bytecode = vec![0u8; 32];
    let original_bytecode = bytecode.clone();
    let config = BridgeConfig::default();
    let mut cooling = CoolingProtocol::new();
    let mut state = KernelState { current_stability: 0.06 }; 
    let shape = Triangle;

    let bad_ai_logic = LogicSignature::new(0.8, 0.9); 

    let is_valid = GeometricValidator::validate_logic_stability(&state, &bad_ai_logic, &shape);
    assert!(!is_valid);

    if is_valid {
        StructuralGuard::protect_assets(&mut bytecode, &config, &mut state, &mut cooling);
    }

    assert_eq!(
        bytecode, 
        original_bytecode, 
        "خرق الإغلاق: تم تعديل البيانات رغم فشل التحقق المنطقي"
    );
}

#[test]
fn test_legacy_telemetry_adaptation_trigger() {
    let mut monitor = HeartbeatMonitor::new(0.5);
    
    monitor.track_legacy_stress(0.1);
    
    thread::sleep(Duration::from_millis(100));

    let needs_adaptation = monitor.track_legacy_stress(5.0);

    assert!(needs_adaptation);
}

#[test]
fn test_critical_threshold_boundary_integrity() {
    let state_at_boundary = KernelState { current_stability: SECURE_CORE };
    let safe_logic = LogicSignature::new(0.01, 0.01);
    let shape = Triangle;

    let is_valid = GeometricValidator::validate_logic_stability(&state_at_boundary, &safe_logic, &shape);
    
    assert!(!is_valid);
}
