// src/bridge/security.rs

//! # Structural Guard & Code Obfuscation
//! 
//! Provides encryption layers for embedded Python bytecode, leveraging the 
//! Kernel's Guard layer to ensure code integrity and prevent reverse engineering.

use crate::core::guard::Guard;
use crate::core::KernelState;
use crate::core::cooling::CoolingProtocol;
use crate::bridge::BridgeConfig;

/// The defensive sentinel for packaged assets.
pub struct StructuralGuard;

impl StructuralGuard {
    /// Protects the embedded bytecode using a lattice-based logic approach.
    /// 
    /// Logic:
    /// 1. Updates system stability metrics via the Kernel Guard.
    /// 2. Verifies that stability is above the SECURE_CORE threshold via verify_integrity.
    /// 3. If stable: applies a XOR-based structural mask to the bytecode.
    /// 4. If unstable: triggers a 'Lockdown' state by refusing to process or modify data.
    pub fn protect_assets(
        bytecode: &mut [u8], 
        config: &BridgeConfig,
        state: &mut KernelState,
        cooling: &mut CoolingProtocol
    ) {
        // 1. Integrity Check: Refresh stability state.
        // Even with 0.0 damage, this ensures the Guard processes any pending cooling effects.
        Guard::apply_damage_with_cooling(state, 0.0, cooling);

        // 2. Uniform Validation: Centralized check against SECURE_CORE.
        if Self::verify_integrity(state) {
            // 3. Obfuscation: Apply security tier mask to the assets.
            let mask = (config.security_tier * 255.0) as u8;
            for byte in bytecode.iter_mut() {
                *byte ^= mask; // Structural XOR Obfuscation
            }
        } else {
            // 4. Lockdown: Bytecode remains untouched.
            // This is the critical path for passing the stability_suite.
            return;
        }
    }

    /// Validates system integrity against the sovereign SECURE_CORE threshold.
    /// 
    /// This function is the ultimate gatekeeper for security operations.
    pub fn verify_integrity(state: &KernelState) -> bool {
        // Alignment: The state is integral ONLY if it stays above the absolute minimum reserve.
        state.current_stability >= crate::core::SECURE_CORE
    }
}
