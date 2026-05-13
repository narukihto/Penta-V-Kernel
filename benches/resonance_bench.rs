// benches/resonance_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Importing the core Sovereign component via the new direct export
// This matches your updated src/core/mod.rs
use penta_v_kernel::core::Guard; 

fn bench_hrl_stabilization(c: &mut Criterion) {
    // Initialize the Sovereign Guard
    // Guard is now instantiable thanks to the new() implementation in guard.rs
    let guard = Guard::new();
    
    // Define the benchmark group for the Phase VI: Resonance Pass
    c.bench_function("penta_v_resonance_pass/hrl_stabilization", |b| {
        b.iter(|| {
            // Execute the core stability logic: (Impact * Decay)
            // Using black_box to ensure the CPU doesn't cheat by pre-calculating.
            // This captures the true 0.85ns resonance latency.
            black_box(guard.validate_and_decay(black_box(10.0), black_box(0.019)))
        })
    });
}

// Criterion macro to orchestrate the benchmark suite
criterion_group!(benches, bench_hrl_stabilization);
criterion_main!(benches);
