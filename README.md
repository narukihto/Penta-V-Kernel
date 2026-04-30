# 🛡️ Penta-V Kernel (Penta-V-Core)
**The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**


> "In the chaos of data, geometry is the only truth. The Penta-V Kernel doesn't just manage load; it redefines the physics of digital survival." — **The First Architect**

---

## 🏛️ Overview
The **Penta-V Kernel** is a mission-critical infrastructure component built in **Rust**. It introduces a paradigm shift in system engineering: **Geometric Load Balancing**. 

Instead of traditional linear queuing, Penta-V treats system stressors as "Deficits" and dissipates them across a variable number of **Geometric Poles**. By dynamically transitioning between shapes—from the foundational Triangle to the asymptotic perfection of the Circle—the kernel ensures that no system shock can compromise the `SECURE_CORE`.

---

## ✅ System Integrity & Automated Testing
Our architectural stability is verified through rigorous automated suites. Every commit triggers a full geometric stress simulation via GitHub Actions.

* **Distributed Mesh:** `test_mesh_pulse_telemetry_integrity` **PASSED** ✅
* **Geometric Efficiency:** `test_efficiency_comparison` **PASSED** ✅
* **Stress Resilience:** `test_decagon_stress_attack` **PASSED** ✅
* **Handshake Security:** `test_mesh_critical_handshake_security` **PASSED** ✅

---

## 🛰️ Phase IV: Distributed Geometric Mesh
In version **v0.2.0**, we introduced the **Distributed Mesh Protocol**. Kernels can now communicate their stability states across a network, allowing for proactive load sharing and systemic immunity synchronization.

*   **Stability Packets:** Zero-allocation telemetry for inter-kernel sync.
*   **Mesh Heartbeats:** Proactive health signals to prevent localized core collapse.
*   **Secure Peering:** Trust-boundary enforcement via the `secure_gate` protocol.

---

## 📊 The Hierarchy of Immunity


| Shape | Poles ($N$) | Immunity ($\Phi$) | Tier | Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **Triangle** | 3 | 1.00 | Foundation | Idle / Low-Power |
| **Hexagon** | 6 | 2.00 | Balanced | Standard Load |
| **Nonagon** | 9 | 3.00 | Triple-Stability | High-Traffic |
| **Dodecagon** | 12 | 4.00 | Quad-Stability | Critical Stress |
| **Circle** | $\infty$ | $\infty$ | **The Shield** | Asymptotic Defense |

---

## 🚀 Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
penta_v_kernel = "0.2.0"
```

### Basic Mesh Usage:
```rust
use penta_v_kernel::mesh::{MeshNode, MeshPulse};

fn main() {
    let mut node = MeshNode::new(0xAA55, true);
    
    // Generate a stability heartbeat for the mesh
    let pulse = node.generate_pulse();
    
    println!("Node {} Stability: {}", pulse.node_id, pulse.stability_score);
}
```

---

## 📜 Roadmap
- [x] **Phase I**: Core Geometric Logic & Pole Mathematics.
- [x] **Phase II**: Real-time Thermal-Aware Decay Scaling & Guard Protection.
- [x] **Phase IV**: Penta-V Distributed Mesh (Inter-kernel communication).
- [x] **Phase V**: Hardware-level integration (FPGA / ASIC acceleration).

---

⚖️ **License**
Distributed under the Apache License 2.0. Created by **Isaac Andrew (The First Architect)**.
