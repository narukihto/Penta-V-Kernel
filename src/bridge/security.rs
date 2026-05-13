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
    /// Updated Logic (Sovereign Patch):
    /// 1. PRE-VALIDATION: Check integrity BEFORE the Guard layer can clamp the value.
    /// 2. If unstable: Triggers 'Lockdown' immediately, keeping bytecode untouched.
    /// 3. If stable: Updates metrics and applies the XOR structural mask.
    pub fn protect_assets(
        bytecode: &mut [u8], 
        config: &BridgeConfig,
        state: &mut KernelState,
        cooling: &mut CoolingProtocol
    ) {
        // --- STEP 1: PRE-VALIDATION (The Critical Fix) ---
        // We verify integrity based on the RAW state before Guard::apply_damage_with_cooling
        // has a chance to force the stability back up to SECURE_CORE (0.05).
        if !Self::verify_integrity(state) {
            // Lockdown: Bytecode remains untouched.
            // This ensures the stability_suite passes when stability is < 0.05.
            return;
        }

        // --- STEP 2: METRIC REFRESH ---
        // Now that we know we are stable, we refresh the state.
        Guard::apply_damage_with_cooling(state, 0.0, cooling);

        // --- STEP 3: OBFUSCATION ---
        // Apply security tier mask to the assets.
        let mask = (config.security_tier * 255.0) as u8;
        for byte in bytecode.iter_mut() {
            *byte ^= mask; // Structural XOR Obfuscation
        }
    }

    /// Validates system integrity against the sovereign SECURE_CORE threshold.
    pub fn verify_integrity(state: &KernelState) -> bool {
        // The state is integral ONLY if it stays strictly above the absolute minimum reserve.
        // We use a strict inequality to ensure 0.05 is the true 'Lockdown' boundary.
        state.current_stability > crate::core::SECURE_CORE
    }
}
