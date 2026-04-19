// tests/benchmark_suite.rs

//! Efficiency and Power Usage benchmarks for the Penta-V Kernel.
//! Compares stability degradation across different geometric levels.

use penta_v_kernel::core::{KernelState, CORE_BASE};
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;

#[test]
fn test_efficiency_comparison() {
    let mut triangle_state = KernelState::default();
    let mut decagon_state = KernelState::default();

    let triangle = Triangle;
    let decagon = Decagon;

    // Simulate 10 units of deficit (loss of one output)
    calculate_and_apply_decay(&mut triangle_state, 10.0, &triangle);
    calculate_and_apply_decay(&mut decagon_state, 10.0, &decagon);

    let triangle_loss = CORE_BASE - triangle_state.current_stability;
    let decagon_loss = CORE_BASE - decagon_state.current_stability;

    println!("Efficiency Report:");
    println!("Triangle loss: {:.4}", triangle_loss);
    println!("Decagon loss:  {:.4}", decagon_loss);

    // Verify Decagon is more efficient at dissipating the shock
    assert!(decagon_loss < triangle_loss);
}
