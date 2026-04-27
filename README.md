# 🛡️ Penta-V Kernel (Penta-V-Core)
**The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Architecture: Geometric](https://img.shields.io/badge/Architecture-Geometric-blue.svg)](#)
[![License: Apache--2.0](https://img.shields.io/badge/License-Apache--2.0-red.svg)](#)
[![Tests: Passing](https://img.shields.io/badge/Tests-Passing-brightgreen.svg)](#)

> "In the chaos of data, geometry is the only truth. The Penta-V Kernel doesn't just manage load; it redefines the physics of digital survival." — **The First Architect**

---

## 🏛️ Overview
The **Penta-V Kernel** is a mission-critical infrastructure component built in **Rust**. It introduces a paradigm shift in system engineering: **Geometric Load Balancing**. 

Instead of traditional linear queuing, Penta-V treats system stressors as "Deficits" and dissipates them across a variable number of **Geometric Poles**. By dynamically transitioning between shapes—from the foundational Triangle to the asymptotic perfection of the Circle—the kernel ensures that no system shock can compromise the `SECURE_CORE`.

---

## ✅ System Integrity: Latest Test Results
Our latest architectural simulation confirms the absolute stability of the core. All core modules have achieved **Green Status** under extreme load scenarios:

* **Core Stability Logic:** `test_decay_calculation_logic` **PASSED** ✅ (Verified 50% Guard baseline protection).
* **Geometric Efficiency:** `test_efficiency_comparison` **PASSED** ✅ (Decagon efficiency validated against Triangle).
* **Stress Resilience:** `test_decagon_stress_attack` **PASSED** ✅ (System maintained stability under high deficit).
* **Absolute Defense:** `test_total_collapse_protection` **PASSED** ✅ (SECURE_CORE triggered at 5000 units of load).
* **Memory Safety:** `test_nan_resilience` **PASSED** ✅ (Zero corruption on invalid data input).

---

## 💎 Core Philosophy: Why Geometry?
Traditional systems fail because they are "flat." When a spike hits, they break or throttle. Penta-V uses **Structural Immunity**. 

By increasing the number of poles ($N$), we increase the **Immunity Factor ($\Phi$)**, effectively "cooling" the system by distributing heat (load) across a wider geometric area.


## 📊 The Hierarchy of Immunity
The kernel transitions through geometric tiers based on real-time telemetry:

| Shape | Poles ($N$) | Immunity ($\Phi$) | Tier | Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **Triangle** | 3 | 1.00 | Foundation | Idle / Low-Power State |
| **Hexagon** | 6 | 2.00 | Balanced | Standard Operating Load |
| **Nonagon** | 9 | 3.00 | Triple-Stability | High-Traffic Surge |
| **Dodecagon** | 12 | 4.00 | Quad-Stability | Critical Stress Event |
| **Circle** | $\infty$ | $\infty$ | **The Shield** | Asymptotic Defense |

---

## ⚔️ Comparisons: Penta-V vs. Traditional Logic

| Feature | Standard Load Balancers | Penta-V Kernel |
| :--- | :--- | :--- |
| **Logic** | Linear (First-In, First-Out) | **Geometric (Spatial Dissipation)** |
| **Adaptability** | Reactive (Scaling Up) | **Proactive (Reshaping Structure)** |
| **Safety** | Exception Handling | **Inherent Structural Immunity** |
| **Efficiency** | High Overhead | **Zero-Cost Abstractions (Rust)** |
| **Protection** | Best-effort | **Guaranteed SECURE_CORE (0.05)** |

---

## 🛠️ Internal Architecture

### 1. Shape Engine (`src/shapes/`)
A modular library of geometric primitives. Every shape implements the `GeometricBalancer` trait, making the system infinitely extensible.

### 2. The Guard (`src/core/guard.rs`)
The final arbiter of stability. It enforces the **SECURE_CORE** threshold. If a calculation would drop stability below this point, the Guard triggers an immediate geometric escalation and damage mitigation.

### 3. Reactive Cooling (`src/core/cooling.rs`)
Simulates digital cooling by dynamically adjusting the system's thermal state. It integrates a reduction factor that scales based on geometric efficiency.

---

## 🚀 Quick Start (Rust)

Add to your `Cargo.toml`:
```toml
[dependencies]
penta_v_kernel = "1.0.0"
Basic usage:
Rust
use penta_v_kernel::shapes::{Circle, GeometricBalancer};

fn main() {
    let shield = Circle;
    let deficit = 500.0;
    
    // Calculate how the Circle dissipates a massive shock
    let impact = (deficit * 0.02) / shield.immunity_factor();
    
    println!("System Impact: {}", impact); // Result: 0.0 (Asymptotic Immunity)
}
```
https://github.com/narukihto/Penta-V-Kernel.git

📜 Roadmap & Advanced Capabilities
[x] Phase I: Core Geometric Logic & Pole Mathematics.

[x] Phase II: Real-time Thermal-Aware Decay Scaling & Guard Protection

[ ] Phase IV: Penta-V Distributed Mesh (Inter-kernel communication).

[ ] Phase V: Hardware-level integration (FPGA / ASIC acceleration).

⚖️ License
Distributed under the Apache License 2.0. Created by Isaac Andrew (The First Architect).

"Systems do not fail because of load; they fail because of poor architecture. Geometry is the remedy."
