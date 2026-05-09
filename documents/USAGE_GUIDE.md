Markdown# 📖 Penta-V Sovereign Usage Guide (v0.4.0)
**The Definitive Handbook for Every Architect, Engineer, and Developer.**

This guide provides specialized implementation protocols for different engineering disciplines. Penta-V is not a tool; it is a structural mandate.

---

## 🐍 1. For Python & Data Science Architects
*Focus: Logic Sanitization, High-Performance Bridges, and AI-Shielding.*

Since Penta-V provides pre-compiled binaries via `Maturin`, your focus is on wrapping unstable AI or high-load logic within the **Sovereign Bridge**.

### A. Protecting AI-Generated Code
Use the `validator.rs` logic exposed to Python to ensure AI outputs don't drift.
```python
import penta_v_kernel
from penta_v_kernel.bridge import LogicSignature, SovereignPacker

# Wrap autonomous logic
def shielded_ai_task(data):
    # Verify the geometric integrity before processing
    sig = LogicSignature(data, stability_threshold=0.85)
    if sig.is_valid():
        packer = SovereignPacker()
        return packer.pack_sovereign_data(data)
    else:
        # Trigger Geometric Lockdown via guard.rs
        raise SystemError("AI Logic Rejected: Geometric Incoherence")
```
B. High-Speed Data PurificationLeverage cleaner.rs to scrub massive datasets (Polars-integrated).Python# Purify data through the Hexagon Pole for balanced stability
purified_data = penta_v_kernel.purify(raw_data, shape="hexagon")
🦀 2. For Rust Systems EngineersFocus: Low-level Optimization, Geometric Shapes, and Memory Safety.You are working directly with the core and shapes modules. Your goal is to maximize throughput and maintain the SECURE_CORE.A. Implementing Custom Geometric DefenseUtilize the shapes directory to select your defensive posture.Rustuse penta_v_kernel::shapes::dodecagon::Dodecagon;
use penta_v_kernel::core::guard::Guard;

fn secure_operation() {
    let mut defender = Dodecagon::new();
    let status = Guard::enforce(&mut defender);
    
    if status.is_optimal() {
        // High-stress execution allowed
    }
}
B. Managing Computational HeatUse cooling.rs to prevent thermal logic collapse during long-running processing cycles.Rustuse penta_v_kernel::core::cooling::ThermalManager;

let mut thermal = ThermalManager::new();
thermal.apply_cooling(0.05); // Reduce logic heat by 5%
🛡️ 3. For Cyber-Security & DevOps EngineersFocus: DDoS Mitigation, Distributed Mesh, and Integrity Tests.You manage the telemetry.rs and mesh.rs layers to ensure the system survives volumetric attacks.A. Activating the Distributed MeshSynchronize stability across multiple kernels.Rustuse penta_v_kernel::mesh::MeshNode;

let mut node = MeshNode::initialize(0xAA55);
node.broadcast_pulse(); // Send heartbeat to the network
B. Running Integrity AuditsPenta-V comes with a built-in stability_suite.rs. Always run this in your CI/CD pipeline.Bash# Verify system resilience against decagon-level stress attacks
cargo test --test stability_suite test_decagon_stress_attack
⚡ 4. For Embedded & IoT Architects (no-std)Focus: Minimal Footprint, Jitter Control, and Thermal Logic.Use the utils module and calculator.rs to manage stability on constrained hardware.Jitter Control: Implement resonance.rs to unity the processing frequency.Deterministic Calculations: Rely on calculator.rs for NaN resilience in sensor data.📊 5. Summary: Which Module to Use?NeedModulePathLogic Cleanupcleaner.rssrc/processing/cleaner.rsDDoS Defenseguard.rssrc/core/guard.rsAI Validationvalidator.rssrc/bridge/validator.rsTelemetrytelemetry.rssrc/bridge/telemetry.rsNetwork Syncmesh.rssrc/mesh.rsStability Proofresonance.rssrc/resonance.rs🚀 Final Verification ProcedureBefore deploying Penta-V to production, every architect must pass the Sovereign Trinity Test:Initialization Test: test_initialization (Ensures core is alive).Resonance Test: test_resonance_identity_fidelity (Ensures no data loss).Stress Test: test_total_collapse_protection (Ensures the Shield holds).Bash# Execute the Trinity Test
cargo test
"The code is not just executed; it is governed." — The First Architect
