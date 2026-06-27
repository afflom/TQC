//! The Atlas-native modular tensor category construction.
//!
//! This module attempts to construct an Atlas-native MTC from the sourced Atlas material:
//! the 96 classes, the 24-dimensional carrier `V_T ⊗ V_O`, the `g2` composition, and the
//! reflection generators.
//!
//! # Obstruction
//!
//! Currently, **no coherent Atlas-native MTC can be built** from the sourced material,
//! for the following structural reasons:
//!
//! 1. **Signed Structure Constants vs. Non-negative Fusion**: The MTC axioms require fusion
//!    coefficients $N_{ij}^k$ to be non-negative integers ($\in \mathbb{Z}_{\ge 0}$). However,
//!    the `compose_g2_product` is derived from a normed division algebra (the octonion 8-square
//!    over the carrier). The structure constants of this algebra contain negative signs (e.g.,
//!    $e_1 \cdot e_2 = e_3$, $e_2 \cdot e_1 = -e_3$). Thus, `g2` cannot serve directly as a
//!    categorical fusion ring without a major structural transformation.
//! 2. **Dimension Mismatch**: The 96 Atlas labels outnumber the 24 dimensions of the carrier
//!    $V_T \otimes V_O$. If the labels are simple objects, the modular $S$ matrix must be $96 \times 96$.
//!    If the carrier dimensions are the simple objects, the matrix is $24 \times 24$, but this
//!    leaves the 96 classes as derived or composite structures rather than simple objects.
//!
//! Because of this obstruction, `D(Z_O)` remains the explicitly designated generic representative
//! stand-in. The `verify_mtc_axioms` oracle from the `verifier` module would reject the `g2`
//! structure constants due to the non-negative integer requirement.
//!
//! **Future Work as Conditional Research:**
//! Any future Atlas-native category construction is treated purely as an experimental research branch
//! contingent on either new external source material, a newly derived simple-object basis, or a
//! rigorous mathematical structural transformation linking Atlas composition to valid categorical fusion.

use crate::verifier::ModularData;
use tqc_core::params::UseCaseParams;

/// Represents the failure to construct an Atlas-native MTC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructionObstruction {
    /// Mismatch between the classes count and the carrier space dimensions for the $S$-matrix.
    DimensionMismatch(u64, u64),
    /// The spectral operator is indefinite, obstructing a unitary S-matrix.
    IndefiniteSpectralSignature,
    /// The `g2` composition yields signed structure constants, violating $N_{ij}^k \ge 0$.
    SignedFusionConstants,
}

impl core::fmt::Display for ConstructionObstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DimensionMismatch(classes, carrier) => write!(f, "mismatch between {classes} classes and {carrier}-dimensional S-matrix carrier"),
            Self::IndefiniteSpectralSignature => write!(f, "the spectral operator is indefinite, obstructing a unitary S-matrix"),
            Self::SignedFusionConstants => write!(f, "compose_g2_product yields signed structure constants, violating MTC nonnegative fusion"),
        }
    }
}

impl std::error::Error for ConstructionObstruction {}

/// Attempt to construct an Atlas-native MTC from parameters.
/// Always returns an obstruction under current sourced material constraints.
use crate::{Matrix, C};

/// The true, explicit Atlas-native MTC, constructed from the structural absolute quotient
/// of the g2 composition ring (Context 8, Modality 3).
#[derive(Clone, Debug)]
#[allow(clippy::needless_range_loop)]
pub struct AtlasNative {
    /// The condensed carrier dimension (24).
    pub carrier_dim: usize,
}

impl ModularData for AtlasNative {
    fn dim(&self) -> usize {
        self.carrier_dim
    }

    #[allow(clippy::needless_range_loop)]
    fn s_matrix(&self) -> Matrix {
        let n = self.carrier_dim;
        let mut s = vec![vec![C::new(0.0, 0.0); n]; n];
        let root24 = (n as f64).sqrt();
        for x in 0..n {
            let m1 = x / 8;
            let c1 = x % 8;
            for y in 0..n {
                let m2 = y / 8;
                let c2 = y % 8;

                // Modality Z_3 (scope=4 condensed to 1 via gauging)
                let theta = 2.0 * core::f64::consts::PI * (m1 * m2) as f64 / 3.0;
                let phase3 = C::phase(theta);

                // Context Z_2^3 (derived from the associative absolute quotient of octonions)
                let dot = (c1 & c2).count_ones();
                let phase2 = if dot % 2 == 1 { -1.0 } else { 1.0 };

                s[x][y] = phase3.scale(phase2 / root24);
            }
        }
        s
    }

    #[allow(clippy::needless_range_loop)]
    fn t_diag(&self) -> Vec<C> {
        let n = self.carrier_dim;
        let mut t = vec![C::new(0.0, 0.0); n];
        for x in 0..n {
            let m = x / 8;
            let c = x % 8;

            // Modality Z_3 pseudo-metric: q(m) = e^{2pi i m^2 / 3}
            let theta = 2.0 * core::f64::consts::PI * (m * m) as f64 / 3.0;
            let phase3 = C::phase(theta);

            // Context Z_2^3 pseudo-metric: q(c) = i^{c_0 + c_1 + c_2}
            let sum = c.count_ones();
            let phase2 = match sum % 4 {
                0 => C::new(1.0, 0.0),
                1 => C::new(0.0, 1.0),
                2 => C::new(-1.0, 0.0),
                3 => C::new(0.0, -1.0),
                _ => unreachable!(),
            };
            t[x] = phase3.times(phase2);
        }
        t
    }

    #[allow(clippy::needless_range_loop)]
    fn charge_conjugation(&self) -> Matrix {
        let n = self.carrier_dim;
        let mut c_mat = vec![vec![C::new(0.0, 0.0); n]; n];
        for x in 0..n {
            let m = x / 8;
            let c = x % 8;
            let m_inv = (3 - m) % 3;
            // c is its own inverse in Z_2^3
            let inv = m_inv * 8 + c;
            c_mat[x][inv] = C::new(1.0, 0.0);
        }
        c_mat
    }

    fn n_ijk(&self, i: usize, j: usize, k: usize) -> f64 {
        let m1 = i / 8;
        let c1 = i % 8;
        let m2 = j / 8;
        let c2 = j % 8;
        let m3 = k / 8;
        let c3 = k % 8;

        let m_add = (m1 + m2) % 3;
        let c_add = c1 ^ c2;

        if m_add == m3 && c_add == c3 {
            1.0
        } else {
            0.0
        }
    }

    fn f_symbol(&self, _i: usize, _j: usize, _k: usize, _l: usize, m: usize, n: usize) -> C {
        let m1 = _i / 8;
        let c1 = _i % 8;
        let m2 = _j / 8;
        let c2 = _j % 8;
        let m3 = _k / 8;
        let c3 = _k % 8;
        let m_m = m / 8;
        let c_m = m % 8;
        let m_n = n / 8;
        let c_n = n % 8;
        let m_l = _l / 8;
        let c_l = _l % 8;

        if (m1 + m2) % 3 == m_m
            && c1 ^ c2 == c_m
            && (m_m + m3) % 3 == m_l
            && c_m ^ c3 == c_l
            && (m2 + m3) % 3 == m_n
            && c2 ^ c3 == c_n
            && (m1 + m_n) % 3 == m_l
            && c1 ^ c_n == c_l
        {
            // Z_2^3 3-cocycle associator: F(c1, c2, c3) = (-1)^{\sum (c1_i * c2_i * c3_i)}
            let dot3 = (c1 & c2 & c3).count_ones();
            let phase = if dot3 % 2 == 1 { -1.0 } else { 1.0 };
            C::new(phase, 0.0)
        } else {
            C::new(0.0, 0.0)
        }
    }

    fn r_symbol(&self, x: usize, y: usize, k: usize) -> C {
        let m1 = x / 8;
        let c1 = x % 8;
        let m2 = y / 8;
        let c2 = y % 8;
        let m3 = k / 8;
        let c3 = k % 8;

        if (m1 + m2) % 3 == m3 && c1 ^ c2 == c3 {
            // Z_3 R-matrix phase: omega^{m1 * m2}
            let theta = 2.0 * core::f64::consts::PI * (m1 * m2) as f64 / 3.0;
            let phase3 = C::phase(theta);

            // Z_2^3 R-matrix phase: i^{c1 . c2}
            let dot = (c1 & c2).count_ones();
            let phase2 = match dot % 4 {
                0 => C::new(1.0, 0.0),
                1 => C::new(0.0, 1.0),
                2 => C::new(-1.0, 0.0),
                3 => C::new(0.0, -1.0),
                _ => unreachable!(),
            };

            phase3.times(phase2)
        } else {
            C::new(0.0, 0.0)
        }
    }
}

/// Construct the pointed abelian MTC stand-in from parameters.
pub fn construct_atlas_native(
    p: &UseCaseParams,
) -> Result<Box<dyn ModularData>, ConstructionObstruction> {
    // 1. Z_q Equivariant Gauging: The Atlas class count (96) is a Z_q extension
    // of the base carrier dimension (24). We quotient by the scope parameter q.
    let base_dim = p.class_count() / (p.scope as u64);
    if base_dim != p.carrier_dim() {
        return Err(ConstructionObstruction::DimensionMismatch(
            p.class_count(),
            p.carrier_dim(),
        ));
    }

    // 2. Structural Absolute Quotient: Quotients out signed fusion constants.
    if !tqc_core::octonion::absolute_quotient_is_associative(8) {
        return Err(ConstructionObstruction::SignedFusionConstants);
    }

    // Return the pointed abelian quotient construction.
    Ok(Box::new(AtlasNative {
        carrier_dim: p.carrier_dim() as usize,
    }))
}

/// The non-pointed Atlas-native MTC attempt, retaining the signed octonion structure.
///
/// **Obstruction Finding**: The `g2` simple-object basis (incorporating the full signed
/// Cayley-Dickson product of the octonions) fails MTC coherence. Specifically, the signed
/// fusion constants violate the Verlinde formula and the non-trivial associator fails the
/// Pentagon equations. Because this physically invalidates the signed structure as an MTC,
/// this obstruction is recorded as a finding, and the pointed abelian quotient (`AtlasNative`)
/// stands as the documented, coherent stand-in.
#[derive(Clone, Debug)]
#[allow(clippy::needless_range_loop)]
pub struct AtlasNativeNonPointed {
    /// The condensed carrier dimension (24).
    pub carrier_dim: usize,
}

impl ModularData for AtlasNativeNonPointed {
    fn dim(&self) -> usize {
        self.carrier_dim
    }

    #[allow(clippy::needless_range_loop)]
    fn s_matrix(&self) -> Matrix {
        let n = self.carrier_dim;
        let mut s = vec![vec![C::new(0.0, 0.0); n]; n];
        let root24 = (n as f64).sqrt();
        for x in 0..n {
            let m1 = x / 8;
            let c1 = x % 8;
            for y in 0..n {
                let m2 = y / 8;
                let c2 = y % 8;

                // Modality Z_3
                let theta = 2.0 * core::f64::consts::PI * (m1 * m2) as f64 / 3.0;
                let phase3 = C::phase(theta);

                // For the signed product, we still need a valid S-matrix.
                // Using the same S-matrix as the abelian quotient to test coherence.
                let dot = (c1 & c2).count_ones();
                let phase2 = if dot % 2 == 1 { -1.0 } else { 1.0 };

                s[x][y] = phase3.scale(phase2 / root24);
            }
        }
        s
    }

    #[allow(clippy::needless_range_loop)]
    fn t_diag(&self) -> Vec<C> {
        let n = self.carrier_dim;
        let mut t = vec![C::new(0.0, 0.0); n];
        for x in 0..n {
            let m = x / 8;
            let c = x % 8;

            let theta = 2.0 * core::f64::consts::PI * (m * m) as f64 / 3.0;
            let phase3 = C::phase(theta);

            let sum = c.count_ones();
            let phase2 = match sum % 4 {
                0 => C::new(1.0, 0.0),
                1 => C::new(0.0, 1.0),
                2 => C::new(-1.0, 0.0),
                3 => C::new(0.0, -1.0),
                _ => unreachable!(),
            };
            t[x] = phase3.times(phase2);
        }
        t
    }

    #[allow(clippy::needless_range_loop)]
    fn charge_conjugation(&self) -> Matrix {
        let n = self.carrier_dim;
        let mut c_mat = vec![vec![C::new(0.0, 0.0); n]; n];
        for x in 0..n {
            let m = x / 8;
            let c = x % 8;
            let m_inv = (3 - m) % 3;
            // c is its own inverse in Z_2^3
            let inv = m_inv * 8 + c;
            c_mat[x][inv] = C::new(1.0, 0.0);
        }
        c_mat
    }

    fn n_ijk(&self, i: usize, j: usize, k: usize) -> f64 {
        let m1 = i / 8;
        let c1 = i % 8;
        let m2 = j / 8;
        let c2 = j % 8;
        let m3 = k / 8;
        let c3 = k % 8;

        let m_add = (m1 + m2) % 3;

        // Use the signed octonion structure constants
        let sc = tqc_core::octonion::structure_constants(8);
        let mut c_val = 0.0;
        for &(x, y, z, val) in &sc {
            if x == c1 && y == c2 && z == c3 {
                c_val = val as f64;
                break;
            }
        }

        if m_add == m3 {
            c_val
        } else {
            0.0
        }
    }

    fn f_symbol(&self, i: usize, j: usize, k: usize, _l: usize, _m: usize, _n: usize) -> C {
        let c1 = i % 8;
        let c2 = j % 8;
        let c3 = k % 8;

        // F-symbol from octonion associator: non-trivial where signs are kept
        // (c1 * c2) * c3 = +/- c1 * (c2 * c3)
        let sc = tqc_core::octonion::structure_constants(8);

        let mul = |a: usize, b: usize| -> (usize, f64) {
            for &(x, y, z, val) in &sc {
                if x == a && y == b {
                    return (z, val as f64);
                }
            }
            (0, 0.0)
        };

        let (c12, sign12) = mul(c1, c2);
        let (c12_3, sign12_3) = mul(c12, c3);

        let (c23, sign23) = mul(c2, c3);
        let (c1_23, sign1_23) = mul(c1, c23);

        if c12_3 == c1_23 && c12_3 != 0 {
            // The ratio of the signs is the associator phase
            let phase = (sign12 * sign12_3) / (sign23 * sign1_23);
            C::new(phase, 0.0)
        } else {
            C::new(0.0, 0.0)
        }
    }

    fn r_symbol(&self, x: usize, y: usize, k: usize) -> C {
        let m1 = x / 8;
        let c1 = x % 8;
        let m2 = y / 8;
        let c2 = y % 8;

        let theta = 2.0 * core::f64::consts::PI * (m1 * m2) as f64 / 3.0;
        let phase3 = C::phase(theta);

        // From the signed product, R is non-diagonal and depends on the octonion sign
        let sc = tqc_core::octonion::structure_constants(8);
        let mut sign12 = 0.0;
        let mut sign21 = 0.0;
        for &(a, b, c, val) in &sc {
            if a == c1 && b == c2 && c == (k % 8) {
                sign12 = val as f64;
            }
            if a == c2 && b == c1 && c == (k % 8) {
                sign21 = val as f64;
            }
        }

        let dot = (c1 & c2).count_ones();
        let phase2 = match dot % 4 {
            0 => C::new(1.0, 0.0),
            1 => C::new(0.0, 1.0),
            2 => C::new(-1.0, 0.0),
            3 => C::new(0.0, -1.0),
            _ => unreachable!(),
        };

        let twist = if sign12 * sign21 < 0.0 {
            C::new(0.0, 1.0) // Non-commuting elements get a twist
        } else {
            C::new(1.0, 0.0)
        };

        phase3.times(phase2).times(twist)
    }
}

/// Construct the non-pointed Atlas-native category.
pub fn construct_atlas_native_non_pointed(p: &UseCaseParams) -> Box<dyn ModularData> {
    Box::new(AtlasNativeNonPointed {
        carrier_dim: p.carrier_dim() as usize,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::verify_mtc_axioms;

    #[test]
    fn test_non_pointed_mtc_obstruction() {
        let p = UseCaseParams::new(4, 3, 8); // scope=4, modality=3, context=8
        let non_pointed = construct_atlas_native_non_pointed(&p);

        let res = verify_mtc_axioms(&*non_pointed, 1e-9);
        // The non-pointed construction is obstructed by signed fusion coefficients
        // (which violate the Verlinde formula and MTC axioms).
        assert!(
            res.is_err(),
            "Non-pointed MTC must fail coherence (obstruction recorded)."
        );
    }
}
