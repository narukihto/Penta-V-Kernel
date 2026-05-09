# 📖 Penta-V Sovereign Usage Guide (v0.4.0)
**The Definitive Handbook for Every Architect, Engineer, and Developer.**

This guide provides specialized implementation protocols for different engineering disciplines. Penta-V is not a tool; it is a structural mandate.

---

## 🐍 1. For Python & Data Science Architects
*Focus: Logic Sanitization, High-Performance Bridges, and AI-Shielding.*

Since Penta-V provides pre-compiled binaries via Maturin, your focus is on wrapping unstable AI or high-load logic within the Sovereign Bridge.

### A. Protecting AI-Generated Code
Use the validator.rs logic exposed to Python to ensure AI outputs don't drift.

import penta_v_kernel
from penta_v_kernel.bridge import LogicSignature, SovereignPacker

def shielded_ai_task(data):
    sig = LogicSignature(data, stability_threshold=0.85)
    if sig.is_valid():
        packer = SovereignPacker()
        return packer.pack_sovereign_data(data)
    else:
        raise SystemError("AI Logic Rejected: Geometric Incoherence")

### B. High-Speed Data Purification
purified_data = penta_v_kernel.purify(raw_data, shape="hexagon")

---

## 🦀 2. For Rust Systems Engineers
*Focus: Low-level Optimization, Geometric Shapes, and Memory Safety.*

You are working directly with the core and shapes modules. Your goal is to maximize throughput and maintain the SECURE_CORE.

### A. Implementing Custom Geometric Defense
use penta_v_kernel::shapes::dodecagon::Dodecagon;
use penta_v_kernel::core::guard::Guard;

fn secure_operation() {
    let mut defender = Dodecagon::new();
    let status = Guard::enforce(&mut defender);
    if status.is_optimal() { /* execution allowed */ }
}

### B. Managing Computational Heat
use penta_v_kernel::core::cooling::ThermalManager;
let mut thermal = ThermalManager::new();
thermal.apply_cooling(0.05);

---

## 🛡️ 3. For Cyber-Security & DevOps Engineers
*Focus: DDoS Mitigation, Distributed Mesh, and Integrity Tests.*

### A. Activating the Distributed Mesh
use penta_v_kernel::mesh::MeshNode;
let mut node = MeshNode::initialize(0xAA55);
node.broadcast_pulse();

### B. Running Integrity Audits
# Verify system resilience against decagon-level stress attacks
cargo test --test stability_suite test_decagon_stress_attack

---

## 📊 5. Summary: Which Module to Use?

Need: Logic Cleanup | Module: cleaner.rs | Path: src/processing/cleaner.rs
Need: DDoS Defense | Module: guard.rs | Path: src/core/guard.rs
Need: AI Validation | Module: validator.rs | Path: src/bridge/validator.rs
Need: Telemetry | Module: telemetry.rs | Path: src/bridge/telemetry.rs
Need: Network Sync | Module: mesh.rs | Path: src/mesh.rs
Need: Stability Proof | Module: resonance.rs | Path: src/resonance.rs

---

## 🚀 Final Verification Procedure
Before deploying Penta-V to production, every architect must pass the Sovereign Trinity Test:
1. Initialization Test: test_initialization
2. Resonance Test: test_resonance_identity_fidelity
3. Stress Test: test_total_collapse_protection

# Execute the Trinity Test
cargo test

"The code is not just executed; it is governed." — The First Architect
