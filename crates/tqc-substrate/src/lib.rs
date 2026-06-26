//! The substrate facade: the single place that imports the holospaces substrate and the
//! uor-addr composition operations.
//!
//! Today it exposes the holospaces content-addressing surface (κ-labels) and re-exports
//! `uor_addr` for the fusion (`g2`) / dual (`f4`) reduction work. Keeping every substrate
//! symbol behind this one crate means a substrate API or revision change has a blast radius
//! of exactly one crate (the math crates stay substrate-free and offline-testable).
//!
//! The dictionary rows realized through this facade (`fusion-g2`, `dual-f4`,
//! `categorical-structure`, `ground-space-protection`, `complex-amplitude-encoding`) are
//! `target` (expected-RED, non-gating) until each is promoted by its BDD scenario.

#![forbid(unsafe_code)]

/// The content-addressing label type from holospaces (`KappaLabel71`).
pub use holospaces::Kappa;
/// The hash-axis selector (Blake3 / Sha256 / Sha3_256 / Keccak256 / Sha512).
pub use holospaces::Axis;

/// Re-export of the realized composition operations, for the fusion/dual reduction.
pub use uor_addr;

/// Address canonical bytes to a content κ on the default (Blake3) axis.
#[must_use]
pub fn kappa(canonical_bytes: &[u8]) -> Kappa {
    holospaces::address(canonical_bytes)
}

/// Verify that bytes re-derive to an expected κ (Law L5: re-derive, never trust).
///
/// The substrate's internal axis-error type is not leaked across the facade boundary; an
/// unknown-axis failure is surfaced as a message.
///
/// # Errors
/// Returns the formatted axis error if the κ axis is unknown.
pub fn verify(canonical_bytes: &[u8], expected: &Kappa) -> Result<bool, String> {
    holospaces::verify(canonical_bytes, expected).map_err(|e| format!("{e:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kappa_is_stable_and_reverifies() {
        let bytes = b"the same content has the same address";
        let k = kappa(bytes);
        assert_eq!(k, kappa(bytes), "addressing must be deterministic");
        assert_eq!(verify(bytes, &k), Ok(true), "content must re-derive to its kappa");
        let other = kappa(b"different content");
        assert_ne!(k, other, "different content must have a different kappa");
    }
}
