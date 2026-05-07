use penta_v_kernel::core::{KernelState, SECURE_CORE};
use penta_v_kernel::core::cooling::CoolingProtocol;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;
use penta_v_kernel::processing::{ProcessingEngine, ProcessingState};
use penta_v_kernel::bridge::{StructuralGuard, BridgeConfig, validator::LogicSignature};
use polars_core::prelude::*;

#[test]
fn test_decagon_stress_attack() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let decagon = Decagon;

    calculate_and_apply_decay(&mut state, 50.0, &decagon, &mut cooling);

    assert!(state.current_stability > SECURE_CORE);
}

#[test]
fn test_thermal_aware_processing_stability() {
    let mut state = ProcessingState::default();
    let mut cooling = CoolingProtocol::new();
    let kernel_state = KernelState::default();
    let signature = LogicSignature::new(0.1, 0.1);
    let shape = Triangle;
    
    let mut df = DataFrame::new(vec![Series::new("metrics", &[1.0, 2.0, 3.0])]).unwrap();

    cooling.state = penta_v_kernel::core::cooling::CoolingState::Active;
    state.data_pressure = 1.0; 

    let _ = ProcessingEngine::dispatch_cleaning_task(
        &mut df, 
        &mut state, 
        &kernel_state, 
        &cooling, 
        &signature, 
        &shape
    );

    assert_eq!(state.data_pressure, 0.5);
    assert_eq!(state.active_poles, 3);
}

#[test]
fn test_security_lockdown_on_instability() {
    let mut bytecode = vec![0u8; 64];
    let original_bytecode = bytecode.clone();
    let config = BridgeConfig::default();
    let mut state = KernelState { current_stability: 0.04 }; 
    let mut cooling = CoolingProtocol::new();

    assert!(!StructuralGuard::verify_integrity(&state));

    StructuralGuard::protect_assets(&mut bytecode, &config, &mut state, &mut cooling);

    assert_eq!(bytecode, original_bytecode);
}

#[test]
fn test_total_collapse_protection() {
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();
    let triangle = Triangle;

    calculate_and_apply_decay(&mut state, 5000.0, &triangle, &mut cooling);

    assert!((state.current_stability - SECURE_CORE).abs() < f64::EPSILON);
}
