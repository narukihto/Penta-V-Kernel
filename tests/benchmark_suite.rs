// tests/benchmark_suite.rs

//! # 🚀 Sovereign Benchmark Suite
//! 
//! Advanced Efficiency, Power Usage, and Processing Throughput benchmarks.
//! Validates the performance advantage of Rust-level Data Cleaning and Sovereign Packaging.

use penta_v_kernel::core::{KernelState, CORE_BASE};
use penta_v_kernel::core::cooling::CoolingProtocol; 
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;
use penta_v_kernel::processing::{PentaCleaner, ProcessingState};
// Removed SovereignPacker to resolve unused_imports warning as per CI logs
use penta_v_kernel::bridge::{BridgeConfig, StructuralGuard};
use polars_core::prelude::*;

#[test]
fn test_geometric_efficiency_comparison() {
    let mut triangle_state = KernelState::default();
    let mut decagon_state = KernelState::default();
    let mut cooling = CoolingProtocol::new();

    let triangle = Triangle;
    let decagon = Decagon;

    // Simulate stability impact across geometric levels
    calculate_and_apply_decay(&mut triangle_state, 10.0, &triangle, &mut cooling);
    calculate_and_apply_decay(&mut decagon_state, 10.0, &decagon, &mut cooling);

    let triangle_loss = CORE_BASE - triangle_state.current_stability;
    let decagon_loss = CORE_BASE - decagon_state.current_stability;

    println!("--- Structural Efficiency Report ---");
    println!("Triangle stability loss: {:.4}", triangle_loss);
    println!("Decagon stability loss:  {:.4}", decagon_loss);

    assert!(decagon_loss < triangle_loss);
}

#[test]
fn test_processing_throughput_benchmark() {
    // 1. Setup a "Messy" high-volume dataset for cleaning
    let s0 = Series::new("node_id", 0..100000);
    let s1 = Series::new("stability_values", vec![Some(1.0), None, Some(f64::NAN), Some(0.5)].into_iter().cycle().take(100000).collect::<Vec<_>>());
    let mut df = DataFrame::new(vec![s0, s1]).unwrap();
    
    let state = ProcessingState {
        data_pressure: 0.9, // Dodecagon stress level
        active_poles: 12,
    };

    println!("--- Penta-Clean Throughput Benchmark ---");
    let start = std::time::Instant::now();
    
    // Execute Rust-level parallel cleaning
    let result = PentaCleaner::geometric_sanitize(&mut df, &state);
    
    let duration = start.elapsed();
    println!("Cleaned 100,000 rows in: {:?}", duration);

    assert!(result.is_ok());
    assert!(duration.as_millis() < 50); // Performance Target: Sub-50ms for 100k rows
}

#[test]
fn test_security_overhead_benchmark() {
    let mut bytecode = vec![0u8; 1024]; // Simulate 1KB of Python bytecode
    let config = BridgeConfig::default();
    let mut state = KernelState::default();
    let mut cooling = CoolingProtocol::new();

    println!("--- Penta-Pack Security Overhead ---");
    let start = std::time::Instant::now();

    // Apply Structural Obfuscation
    StructuralGuard::protect_assets(&mut bytecode, &config, &mut state, &mut cooling);

    let duration = start.elapsed();
    println!("Bytecode obfuscation latency: {:?}", duration);

    // Verify system stability was maintained during protection
    assert!(state.current_stability >= 0.2); 
    assert!(duration.as_micros() < 500); // Should be near-instant
}
