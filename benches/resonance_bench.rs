// benches/resonance_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// Importing the core Sovereign components
use penta_v_kernel::core::guard::Guard; 

fn bench_hrl_stabilization(c: &mut Criterion) {
    // Initialize the Sovereign Core
    let mut guard = Guard::new();
    
    // Define the benchmark group for the Resonance Pass
    c.bench_function("penta_v_resonance_pass/hrl_stabilization", |b| {
        b.iter(|| {
            // Execute the core stability operation.
            // black_box prevents the compiler from optimizing away the logic, 
            // ensuring we measure the true computational cost.
            black_box(guard.validate_and_decay(black_box(10.0), black_box(4.0)))
        })
    });
}

// Criterion macro to set up the benchmark harness
criterion_group!(benches, bench_hrl_stabilization);
criterion_main!(benches);
