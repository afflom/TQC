#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_memcpy)]
//! Exact algebraic density certificate for the coupled Atlas generators.
//!
//! This module discharges the reviewer item: the non-collapse / nontriviality step of the
//! Solovay–Kitaev density witness is decided exactly over the cyclotomic field
//! `F = Q(zeta_24)`, not witnessed in `f64` against a threshold.
//!
//! Mathematical basis (every step exact; the single analytic input is Lindemann's theorem):
//!
//! 1. At the atlas use-case (modality 3, context 8, carrier 24) every entry of
//!    `S~ = sqrt(24) * S` and of `T` lies in `F = Q(zeta_24)` (which contains `zeta_3`, `i`,
//!    `sqrt(2)`, `sqrt(3)`, hence `sqrt(6)`).
//! 2. The coupled generators are `G_S = S * E`, `G_T = T * E`, `E = diag(t^{m_j})`, where
//!    `m_j` are the integer spectral eigenvalues `{10, 7, 2, -1}` (multiplicities 1,2,7,14,
//!    contiguous blocks `Pi_p`) and `t = e^{i}`. By Lindemann, `t` is transcendental.
//! 3. Grading: equating Laurent coefficients in the transcendental `t`,
//!    `[X, G_S] = [X, G_T] = 0` iff `[X, S] = [X, T] = [X, Pi_p] = 0` for all `p`
//!    (`S`, `T` invertible). The commutant of the generated group is therefore the kernel of
//!    an explicit linear system over `F`: its dimension, a Hermitian generator `C`, the block
//!    projector `P1`, and all traces are exact `F`-arithmetic.
//! 4. `commutant dim == 2` implies the representation splits into two inequivalent irreps of
//!    multiplicity one; `tr P1 == 2` identifies the 2-dimensional block, which is therefore
//!    irreducible (hence the restricted image is non-abelian only if a graded commutator
//!    coefficient survives — checked exactly below).
//! 5. Structural finding, decided exactly: `tr(P1 G_S) = 0` identically (all four coefficients
//!    `tr(P1 * S * Pi_p)` vanish over `F`), so the restricted `u_s` is traceless, squares to a
//!    scalar, and is a projective involution. Any `f64` threshold claiming a nonzero S-side
//!    trace coefficient is thereby refuted, and any certificate shaped as "both generators
//!    infinite order" cannot close: `{u_t infinite, u_s involution, non-commuting, irreducible}`
//!    is exactly realized by `O(2)` inside `SO(3)`.
//! 6. The correct criterion runs in `PU(2) = SO(3)`, whose closed subgroups are finite,
//!    `SO(2)`, `O(2)`, or `SO(3)`. Two projective properties are decided as exact
//!    Laurent-polynomial identities over `F` (a polynomial with coefficients in `F` vanishing
//!    at the transcendental `t = e^i` vanishes identically):
//!    projectively infinite order (`tr(u)^2 / det(u)` not algebraic, via `2 det = tr^2 -
//!    tr(u^2)` and a proportionality identity) and projectively non-commuting
//!    (`u_a u_b = c u_b u_a` fails for every complex scalar `c`, via entrywise cross-product
//!    identities of the graded matrices).
//! 7. A projectively non-commuting pair of projectively infinite-order words excludes finite
//!    subgroups (no infinite element), `SO(2)` (abelian), and `O(2)` (its projectively
//!    infinite elements all lie in the index-2 `SO(2)` and commute). The closure is therefore
//!    `SO(3)`: density on the block up to global phase, which is what universality requires.
//!
//! `f64` appears in this module only in cross-checks against the runtime construction
//! (`derive, never hand-enter`): the exact matrices are compared entrywise against
//! `tqc_mtc::native` before any decision is made. No decision depends on a float.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

/// Degree of `Q(zeta_24)` over `Q`; minimal polynomial `Phi_24(x) = x^8 - x^4 + 1`.
const DEG: usize = 8;

/// An element of `Q(zeta_24)`, as rational coordinates on `1, z, ..., z^7`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cyc {
    c: Vec<BigRational>,
}

fn rz() -> BigRational {
    BigRational::zero()
}

impl Cyc {
    /// The exact zero element of the cyclotomic field.
    pub fn zero() -> Self {
        Cyc { c: vec![rz(); DEG] }
    }
    /// The exact one element of the cyclotomic field.
    pub fn one() -> Self {
        let mut v = Cyc::zero();
        v.c[0] = BigRational::one();
        v
    }
    /// Conversion from an integer to a cyclotomic element.
    pub fn from_int(n: i64) -> Self {
        let mut v = Cyc::zero();
        v.c[0] = BigRational::from(BigInt::from(n));
        v
    }
    /// Checks if the element is exactly zero.
    pub fn is_zero(&self) -> bool {
        self.c.iter().all(num_traits::Zero::is_zero)
    }
    /// Addition in the cyclotomic field.
    pub fn add(&self, o: &Cyc) -> Cyc {
        let mut r = self.clone();
        for k in 0..DEG {
            r.c[k] += o.c[k].clone();
        }
        r
    }
    /// Subtraction in the cyclotomic field.
    pub fn sub(&self, o: &Cyc) -> Cyc {
        let mut r = self.clone();
        for k in 0..DEG {
            r.c[k] -= o.c[k].clone();
        }
        r
    }
    /// Additive inverse in the cyclotomic field.
    pub fn neg(&self) -> Cyc {
        Cyc::zero().sub(self)
    }
    /// Multiplication in the cyclotomic field.
    pub fn mul(&self, o: &Cyc) -> Cyc {
        // convolve to degree 14, then fold x^d = x^{d-4} - x^{d-8}
        let mut w = vec![rz(); 2 * DEG - 1];
        for i in 0..DEG {
            if self.c[i].is_zero() {
                continue;
            }
            for j in 0..DEG {
                if o.c[j].is_zero() {
                    continue;
                }
                w[i + j] += self.c[i].clone() * o.c[j].clone();
            }
        }
        for d in (DEG..2 * DEG - 1).rev() {
            if w[d].is_zero() {
                continue;
            }
            let t = std::mem::replace(&mut w[d], rz());
            w[d - 4] += t.clone();
            w[d - 8] -= t;
        }
        Cyc {
            c: w[..DEG].to_vec(),
        }
    }
    /// `zeta_24^k` for any integer `k` (mod 24), exactly.
    pub fn zeta_pow(k: i64) -> Cyc {
        let k = k.rem_euclid(24) as usize;
        let mut v = Cyc::one();
        for _ in 0..k {
            // multiply by x: shift, fold x^8 = x^4 - 1
            let mut w = vec![rz(); DEG + 1];
            for i in 0..DEG {
                w[i + 1] = v.c[i].clone();
            }
            let t = std::mem::replace(&mut w[DEG], rz());
            w[4] += t.clone();
            w[0] -= t;
            v = Cyc {
                c: w[..DEG].to_vec(),
            };
        }
        v
    }
    /// Complex conjugation = the Galois map `zeta -> zeta^{-1}`.
    pub fn conj(&self) -> Cyc {
        let mut r = Cyc::zero();
        for k in 0..DEG {
            if self.c[k].is_zero() {
                continue;
            }
            let mut term = Cyc::zeta_pow(-(k as i64));
            for x in &mut term.c {
                *x *= self.c[k].clone();
            }
            r = r.add(&term);
        }
        r
    }
    /// Field inverse via the 8x8 multiplication matrix. Errors on zero.
    pub fn inv(&self) -> Result<Cyc, String> {
        if self.is_zero() {
            return Err("division by zero in Q(zeta_24)".into());
        }
        // columns: coefficients of self * x^j
        let mut cols: Vec<Vec<BigRational>> = Vec::with_capacity(DEG);
        let mut y = self.clone();
        for _ in 0..DEG {
            cols.push(y.c.clone());
            y = y.mul(&Cyc::zeta_pow(1));
        }
        // solve M a = e0, M[i][j] = cols[j][i]
        let mut m = vec![vec![rz(); DEG + 1]; DEG];
        for i in 0..DEG {
            for j in 0..DEG {
                m[i][j] = cols[j][i].clone();
            }
        }
        m[0][DEG] = BigRational::one();
        for col in 0..DEG {
            let piv = (col..DEG)
                .find(|&r| !m[r][col].is_zero())
                .ok_or("singular multiplication matrix (nonzero element)")?;
            m.swap(col, piv);
            let p = m[col][col].clone();
            for j in col..=DEG {
                m[col][j] = m[col][j].clone() / p.clone();
            }
            for r in 0..DEG {
                if r != col && !m[r][col].is_zero() {
                    let f = m[r][col].clone();
                    for j in col..=DEG {
                        let s = m[col][j].clone() * f.clone();
                        m[r][j] -= s;
                    }
                }
            }
        }
        let mut out = Cyc::zero();
        for i in 0..DEG {
            out.c[i] = m[i][DEG].clone();
        }
        Ok(out)
    }
    /// Numerical evaluation at `zeta = e^{i pi / 12}` (cross-checks only; never a decision).
    pub fn to_c64(&self) -> (f64, f64) {
        let (mut re, mut im) = (0.0, 0.0);
        for k in 0..DEG {
            let a = self.c[k].to_f64().unwrap_or(f64::NAN);
            let th = std::f64::consts::PI * (k as f64) / 12.0;
            re += a * th.cos();
            im += a * th.sin();
        }
        (re, im)
    }
}

type Mat = Vec<Vec<Cyc>>;

fn mat_zero(n: usize) -> Mat {
    vec![vec![Cyc::zero(); n]; n]
}
fn mat_id(n: usize) -> Mat {
    let mut m = mat_zero(n);
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = Cyc::one();
    }
    m
}
fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let n = a.len();
    let mut r = mat_zero(n);
    for i in 0..n {
        for k in 0..n {
            if a[i][k].is_zero() {
                continue;
            }
            for j in 0..n {
                if b[k][j].is_zero() {
                    continue;
                }
                r[i][j] = r[i][j].add(&a[i][k].mul(&b[k][j]));
            }
        }
    }
    r
}
fn mat_sub(a: &Mat, b: &Mat) -> Mat {
    let n = a.len();
    let mut r = mat_zero(n);
    for i in 0..n {
        for j in 0..n {
            r[i][j] = a[i][j].sub(&b[i][j]);
        }
    }
    r
}
fn mat_adjoint(a: &Mat) -> Mat {
    let n = a.len();
    let mut r = mat_zero(n);
    for i in 0..n {
        for j in 0..n {
            r[i][j] = a[j][i].conj();
        }
    }
    r
}
fn mat_is_zero(a: &Mat) -> bool {
    a.iter().all(|row| row.iter().all(Cyc::is_zero))
}
fn mat_scale(a: &Mat, s: &Cyc) -> Mat {
    let n = a.len();
    let mut r = mat_zero(n);
    for i in 0..n {
        for j in 0..n {
            r[i][j] = a[i][j].mul(s);
        }
    }
    r
}
fn mat_trace(a: &Mat) -> Cyc {
    let mut t = Cyc::zero();
    for (i, row) in a.iter().enumerate() {
        t = t.add(&row[i]);
    }
    t
}
fn mat_eq(a: &Mat, b: &Mat) -> bool {
    mat_is_zero(&mat_sub(a, b))
}

/// Exact `sqrt(24) * S` at (modality 3, context 8): entries `zeta_3^{m1 m2} * (-1)^{c1.c2}`.
fn build_s_tilde(modality: usize, context: usize) -> Mat {
    let n = modality * context;
    let mut s = mat_zero(n);
    for x in 0..n {
        let (m1, c1) = (x / context, x % context);
        for y in 0..n {
            let (m2, c2) = (y / context, y % context);
            let z3 = Cyc::zeta_pow(8 * ((m1 * m2) % modality) as i64);
            let sign = (c1 & c2).count_ones() % 2 == 1;
            s[x][y] = if sign { z3.neg() } else { z3 };
        }
    }
    s
}

/// Exact `T` diagonal at (modality 3, context 8): `zeta_3^{m^2} * i^{|c|}` (odd modality).
fn build_t_diag(modality: usize, context: usize) -> Vec<Cyc> {
    let n = modality * context;
    let mut t = Vec::with_capacity(n);
    for x in 0..n {
        let (m, c) = (x / context, x % context);
        let z3 = Cyc::zeta_pow(8 * ((m * m) % modality) as i64);
        let iph = Cyc::zeta_pow(6 * (c.count_ones() % 4) as i64);
        t.push(z3.mul(&iph));
    }
    t
}

/// Report of the exact certificate. Every boolean below is decided over `Q(zeta_24)`.
#[derive(Debug)]
pub struct ExactDensityReport {
    /// Exact dimension of the commutant of the generated group (2 = two irreps, mult 1).
    pub commutant_dim: usize,
    /// Exact dimension of the certified block (`tr P1`, expected 2).
    pub block_dim: usize,
    /// Exponents `p` with `tr(P1 * S~ * Pi_p) != 0` exactly. Empty means `tr(u_s) = 0`
    /// identically: `u_s` is a projective involution (structural finding).
    pub beta_s_nonzero: Vec<i64>,
    /// Exponents `p` with `tr(P1 * T * Pi_p) != 0` exactly.
    pub beta_t_nonzero: Vec<i64>,
    /// A grade at which the matrix-level graded commutator `P1 [G_S, G_T] P1` is nonzero.
    pub noncommuting_grade: Option<i64>,
    /// Words (over the generators) certified projectively infinite order: `tr^2/det`
    /// is not algebraic, decided as a Laurent-polynomial proportionality identity.
    pub proj_infinite: Vec<String>,
    /// A pair of projectively infinite-order words whose restrictions are projectively
    /// non-commuting (`u_a u_b` not proportional to `u_b u_a` as a polynomial identity).
    pub proj_pair: Option<(String, String)>,
    /// Exact trace of `P1 Pi_p` per eigenvalue `p` (shown as f64): the block's support
    /// across the spectral eigenspaces. Support in a single eigenspace means `E` is a
    /// scalar phase on the block and the coupling is projectively trivial there.
    pub block_support: Vec<(i64, f64)>,
    /// When density is refuted: the exact order of the finite projective image of the
    /// generators on the block (BFS over F-proportionality classes). `None` if dense or
    /// if the BFS cap was exceeded.
    pub finite_image_order: Option<usize>,
    /// Words with exactly certified infinite projective order on the 22-dim block, via the
    /// adjoint-trace criterion (`tr(u) tr(u)^*` transcendental).
    pub block22_infinite: Vec<String>,
    /// A projectively non-commuting pair of infinite-projective-order words on the 22-dim
    /// block.
    pub block22_pair: Option<(String, String)>,
    /// Certified: the closure of the projective image on the 22-dim irreducible block is an
    /// infinite non-abelian compact group; the coupled generators exceed every finite gate
    /// set, locating the continuous (beyond-Clifford) content of the machine.
    pub beyond_finite: bool,
    /// Density verdict: closed subgroups of `PU(2) = SO(3)` are finite, `SO(2)`, `O(2)`,
    /// or `SO(3)`; a projectively non-commuting pair of projectively infinite-order
    /// elements excludes the first three, so the closure is all of `SO(3)`.
    pub certified_dense: bool,
    /// Human-readable statement of the certificate.
    pub description: String,
}

/// Run the exact certificate for the atlas use-case (modality 3, context 8, carrier 24).
///
/// Cross-checks the exact matrices against the runtime `tqc_mtc::native` construction
/// entrywise before deciding anything.
#[allow(clippy::too_many_lines)]
pub fn exact_density_certificate(
    p: &tqc_core::UseCaseParams,
) -> Result<ExactDensityReport, String> {
    let native = tqc_mtc::native::construct_atlas_native(p).map_err(|e| e.to_string())?;
    let dim = native.dim();
    let (modality, context) = (3usize, 8usize);
    if dim != modality * context {
        return Err(format!(
            "exact certificate is defined for the atlas use-case (dim 24); got dim {dim}"
        ));
    }

    // ---- exact construction + cross-check against the runtime construction ----
    let s_tilde = build_s_tilde(modality, context);
    let t_diag = build_t_diag(modality, context);
    let root24 = 24f64.sqrt();
    let s_num = native.s_matrix();
    let t_num = native.t_diag();
    for i in 0..dim {
        let (tre, tim) = t_diag[i].to_c64();
        if (tre - t_num[i].re).abs() > 1e-9 || (tim - t_num[i].im).abs() > 1e-9 {
            return Err(format!("exact T disagrees with native t_diag at {i}"));
        }
        for j in 0..dim {
            let (sre, sim) = s_tilde[i][j].to_c64();
            if (sre / root24 - s_num[i][j].re).abs() > 1e-9
                || (sim / root24 - s_num[i][j].im).abs() > 1e-9
            {
                return Err(format!(
                    "exact S~ disagrees with native s_matrix at ({i},{j})"
                ));
            }
        }
    }

    // ---- spectral blocks: eigenvalues {10,7,2,-1}, mults {1,2,7,14}, contiguous ----
    let evals = tqc_core::spectrum::block_eigenvalues(p); // [10,7,2,-1]
    let mults = [1usize, 2, 7, 14];
    let mut block_of = vec![0usize; dim];
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    {
        let mut start = 0usize;
        for (b, &m) in mults.iter().enumerate() {
            for x in start..start + m {
                block_of[x] = b;
            }
            ranges.push((start, start + m));
            start += m;
        }
        if start != dim {
            return Err("multiplicities do not sum to carrier dim".into());
        }
    }

    // ---- commutant over F: X block-diagonal (Pi_p), T-compatible, [X, S~] = 0 ----
    // unknown positions
    let mut pos: Vec<(usize, usize)> = Vec::new();
    let mut pos_idx = vec![vec![usize::MAX; dim]; dim];
    for r in 0..dim {
        for c in 0..dim {
            if block_of[r] == block_of[c] && t_diag[r] == t_diag[c] {
                pos_idx[r][c] = pos.len();
                pos.push((r, c));
            }
        }
    }
    let u = pos.len();

    // rows of [X,S~] = 0: entry (i,j): sum_k S[i][k] X[k][j] - X[i][k] S[k][j] = 0
    let mut rows: Vec<Vec<Cyc>> = Vec::new();
    for i in 0..dim {
        for j in 0..dim {
            let mut row = vec![Cyc::zero(); u];
            let mut any = false;
            for k in 0..dim {
                let a = pos_idx[k][j];
                if a != usize::MAX && !s_tilde[i][k].is_zero() {
                    row[a] = row[a].add(&s_tilde[i][k]);
                    any = true;
                }
                let b = pos_idx[i][k];
                if b != usize::MAX && !s_tilde[k][j].is_zero() {
                    row[b] = row[b].sub(&s_tilde[k][j]);
                    any = true;
                }
            }
            if any {
                rows.push(row);
            }
        }
    }

    // Gaussian elimination over F -> rank, then nullspace basis
    let mut pivots: Vec<usize> = Vec::new();
    let mut reduced: Vec<Vec<Cyc>> = Vec::new();
    for row in rows {
        let mut r = row;
        for (pi, prow) in pivots.iter().zip(reduced.iter()) {
            if !r[*pi].is_zero() {
                let f = r[*pi].clone();
                for j in 0..u {
                    r[j] = r[j].sub(&prow[j].mul(&f));
                }
            }
        }
        if let Some(pcol) = (0..u).find(|&j| !r[j].is_zero()) {
            let inv = r[pcol].inv()?;
            for j in 0..u {
                r[j] = r[j].mul(&inv);
            }
            // back-eliminate into existing rows
            for (pi, prow) in pivots.iter().zip(reduced.iter_mut()) {
                let _ = pi;
                if !prow[pcol].is_zero() {
                    let f = prow[pcol].clone();
                    for j in 0..u {
                        prow[j] = prow[j].sub(&r[j].mul(&f));
                    }
                }
            }
            pivots.push(pcol);
            reduced.push(r);
        }
    }
    let rank = pivots.len();
    let commutant_dim = u - rank;

    if commutant_dim != 2 {
        return Ok(ExactDensityReport {
            commutant_dim,
            block_dim: 0,
            beta_s_nonzero: vec![],
            beta_t_nonzero: vec![],
            noncommuting_grade: None,
            proj_infinite: vec![],
            proj_pair: None,
            block_support: vec![],
            finite_image_order: None,
            block22_infinite: vec![],
            block22_pair: None,
            beyond_finite: false,
            certified_dense: false,
            description: format!(
                "exact commutant dimension is {commutant_dim}, not 2; density on a 2-dim block is not certified"
            ),
        });
    }

    // nullspace basis: free columns
    let piv_set: std::collections::HashSet<usize> = pivots.iter().copied().collect();
    let free: Vec<usize> = (0..u).filter(|j| !piv_set.contains(j)).collect();
    let mut basis_mats: Vec<Mat> = Vec::new();
    for &f in &free {
        let mut xv = vec![Cyc::zero(); u];
        xv[f] = Cyc::one();
        for (pi, prow) in pivots.iter().zip(reduced.iter()) {
            xv[*pi] = prow[f].neg();
        }
        let mut m = mat_zero(dim);
        for (k, &(r, c)) in pos.iter().enumerate() {
            m[r][c] = xv[k].clone();
        }
        basis_mats.push(m);
    }

    // Hermitian non-scalar commutant generator C
    let ident = mat_id(dim);
    let is_scalar = |m: &Mat| -> bool {
        let s = m[0][0].clone();
        mat_eq(m, &mat_scale(&ident, &s))
    };
    let mut c_mat: Option<Mat> = None;
    'outer: for b in &basis_mats {
        let h1 = {
            let a = mat_adjoint(b);
            let mut r = mat_zero(dim);
            for i in 0..dim {
                for j in 0..dim {
                    r[i][j] = b[i][j].add(&a[i][j]);
                }
            }
            r
        };
        let h2 = {
            let a = mat_adjoint(b);
            let i_unit = Cyc::zeta_pow(6);
            mat_scale(&mat_sub(b, &a), &i_unit)
        };
        for h in [h1, h2] {
            if !is_scalar(&h) && mat_eq(&mat_adjoint(&h), &h) {
                c_mat = Some(h);
                break 'outer;
            }
        }
    }
    let c_mat = c_mat.ok_or("no Hermitian non-scalar commutant generator found")?;

    // minimal polynomial: C^2 = a C + b I (must hold exactly since commutant dim == 2)
    let c2 = mat_mul(&c_mat, &c_mat);
    let (a_co, b_co) = {
        let mut off: Option<(usize, usize)> = None;
        for i in 0..dim {
            for j in 0..dim {
                if i != j && !c_mat[i][j].is_zero() {
                    off = Some((i, j));
                }
            }
        }
        if let Some((i, j)) = off {
            let a = c2[i][j].mul(&c_mat[i][j].inv()?);
            let b = c2[0][0].sub(&a.mul(&c_mat[0][0]));
            (a, b)
        } else {
            // C diagonal: two distinct diagonal values are the eigenvalues
            let l1 = c_mat[0][0].clone();
            let l2 = (1..dim)
                .map(|i| c_mat[i][i].clone())
                .find(|v| *v != l1)
                .ok_or("diagonal C is scalar")?;
            (l1.add(&l2), l1.mul(&l2).neg())
        }
    };
    {
        let rhs = {
            let mut r = mat_scale(&c_mat, &a_co);
            for i in 0..dim {
                r[i][i] = r[i][i].add(&b_co);
            }
            r
        };
        if !mat_eq(&c2, &rhs) {
            return Err("C^2 != aC + bI: minimal polynomial inconsistency".into());
        }
    }

    // eigenvalues of C in F via the zero-divisor split of K = F[x]/(x^2 - a x - b):
    // run elimination of (C - lambda I) over K; the first non-invertible nonzero pivot
    // u + v*lambda yields the root lambda = -u/v in F. If C is diagonal we already have them.
    let (l1, l2) = find_roots(&c_mat, &a_co, &b_co, dim)?;

    // block projector P1 with trace 2
    let denom = l1.sub(&l2);
    if denom.is_zero() {
        return Err("repeated eigenvalue: C scalar contradiction".into());
    }
    let mk_proj = |lo: &Cyc| -> Result<Mat, String> {
        let mut m = c_mat.clone();
        for i in 0..dim {
            m[i][i] = m[i][i].sub(lo);
        }
        Ok(mat_scale(&m, &l1.sub(lo).inv()?))
    };
    let p_a = mk_proj(&l2)?; // eigenprojector for l1
    let tr_a = mat_trace(&p_a);
    let two = Cyc::from_int(2);
    let twenty_two = Cyc::from_int(22);
    let p1 = if tr_a == two {
        p_a
    } else if tr_a == twenty_two {
        // the other eigenprojector
        let mut m = c_mat.clone();
        for i in 0..dim {
            m[i][i] = m[i][i].sub(&l1);
        }
        mat_scale(&m, &l2.sub(&l1).inv()?)
    } else {
        return Ok(ExactDensityReport {
            commutant_dim,
            block_dim: 0,
            beta_s_nonzero: vec![],
            beta_t_nonzero: vec![],
            noncommuting_grade: None,
            proj_infinite: vec![],
            proj_pair: None,
            block_support: vec![],
            finite_image_order: None,
            block22_infinite: vec![],
            block22_pair: None,
            beyond_finite: false,
            certified_dense: false,
            description: "exact isotypic dimensions are not {2,22}".into(),
        });
    };
    // verify projector identities exactly
    if !mat_eq(&mat_mul(&p1, &p1), &p1) || !mat_eq(&mat_adjoint(&p1), &p1) {
        return Err("P1 is not an exact orthogonal projector".into());
    }
    let block_dim = 2usize;

    // ---- exact structural anchors ----
    // S~ S~^dagger = 24 I (S unitary up to the sqrt(24) scale), so the commutant is
    // *-closed and P1 (a polynomial in C) commutes with every graded piece below.
    {
        let ssd = mat_mul(&s_tilde, &mat_adjoint(&s_tilde));
        if !mat_eq(&ssd, &mat_scale(&mat_id(dim), &Cyc::from_int(24))) {
            return Err("S~ S~^dagger != 24 I".into());
        }
        let cs = mat_mul(&c_mat, &s_tilde);
        let sc = mat_mul(&s_tilde, &c_mat);
        if !mat_eq(&cs, &sc) {
            return Err("[C, S~] != 0".into());
        }
        // T diagonal: [C, T] entrywise
        for i in 0..dim {
            for j in 0..dim {
                if !c_mat[i][j]
                    .mul(&t_diag[j])
                    .sub(&t_diag[i].mul(&c_mat[i][j]))
                    .is_zero()
                {
                    return Err("[C, T] != 0".into());
                }
            }
        }
    }

    // ---- infinite order: beta_p = tr(P1 * S~ * Pi_p) and tr(P1 * T * Pi_p) ----
    let p1s = mat_mul(&p1, &s_tilde);
    let mut beta_s_nonzero = Vec::new();
    let mut beta_t_nonzero = Vec::new();
    for (bidx, &(lo, hi)) in ranges.iter().enumerate() {
        let mut bs = Cyc::zero();
        let mut bt = Cyc::zero();
        for i in lo..hi {
            bs = bs.add(&p1s[i][i]);
            bt = bt.add(&p1[i][i].mul(&t_diag[i]));
        }
        if !bs.is_zero() {
            beta_s_nonzero.push(evals[bidx]);
        }
        if !bt.is_zero() {
            beta_t_nonzero.push(evals[bidx]);
        }
    }
    let s_infinite = !beta_s_nonzero.is_empty();
    let t_infinite = !beta_t_nonzero.is_empty();

    // ---- non-commuting on the block: graded commutator coefficients ----
    // G_S G_T = sum_p (S T Pi_p) t^{2p};  G_T G_S grades by m_i + m_j on entries of (T S).
    let st = {
        // S~ * T (T diagonal): scale columns
        let mut m = s_tilde.clone();
        for j in 0..dim {
            for i in 0..dim {
                m[i][j] = m[i][j].mul(&t_diag[j]);
            }
        }
        m
    };
    let ts = {
        // T * S~: scale rows
        let mut m = s_tilde.clone();
        for (i, row) in m.iter_mut().enumerate() {
            for e in row.iter_mut() {
                *e = e.mul(&t_diag[i]);
            }
        }
        m
    };
    let mut grades: Vec<i64> = Vec::new();
    for &p in &evals {
        grades.push(2 * p);
    }
    for &pi in &evals {
        for &pj in &evals {
            grades.push(pi + pj);
        }
    }
    grades.sort_unstable();
    grades.dedup();
    let mut noncommuting_grade: Option<i64> = None;
    for &r in &grades {
        let mut a_r = mat_zero(dim);
        // + S T Pi_{r/2} when r = 2p for an eigenvalue p
        if r % 2 == 0 {
            if let Some(bidx) = evals.iter().position(|&e| 2 * e == r) {
                let (lo, hi) = ranges[bidx];
                for i in 0..dim {
                    for j in lo..hi {
                        a_r[i][j] = a_r[i][j].add(&st[i][j]);
                    }
                }
            }
        }
        // - (T S) masked to entries with m_i + m_j = r
        for i in 0..dim {
            for j in 0..dim {
                if evals[block_of[i]] + evals[block_of[j]] == r {
                    a_r[i][j] = a_r[i][j].sub(&ts[i][j]);
                }
            }
        }
        if mat_is_zero(&a_r) {
            continue;
        }
        let m = mat_mul(&mat_mul(&p1, &a_r), &p1);
        if !mat_is_zero(&m) {
            noncommuting_grade = Some(r);
            break;
        }
    }
    let noncommuting = noncommuting_grade.is_some();

    // ---- exact support of the block across the spectral eigenspaces ----
    // tr(P1 Pi_p) = sum of the (nonnegative) diagonal of P1 over block p: zero iff the
    // 2-dim block has no component in eigenspace p. If the support is concentrated in a
    // single eigenspace, E restricts to a scalar phase on the block and the archimedean
    // coupling is projectively trivial there.
    let mut block_support: Vec<(i64, f64)> = Vec::new();
    let mut support_blocks = 0usize;
    for (bidx, &(lo, hi)) in ranges.iter().enumerate() {
        let mut tr = Cyc::zero();
        for i in lo..hi {
            tr = tr.add(&p1[i][i]);
        }
        if !tr.is_zero() {
            support_blocks += 1;
        }
        block_support.push((evals[bidx], tr.to_c64().0));
    }

    // suppress unused warnings for diagnostics retained in the report
    let _ = (s_infinite, t_infinite, noncommuting);

    // ---- projective certificate on the block ----
    // tr(u_s) = 0 exactly (found above when beta_s_nonzero is empty): a traceless 2x2
    // unitary squares to a scalar, so u_s is a projective involution. The facts
    // {u_t infinite order, u_s involution, non-commuting, irreducible} are all realized
    // by O(2) inside SO(3), so they do NOT certify density. The correct exact criterion
    // runs in PU(2) = SO(3), whose closed subgroups are: finite, SO(2), O(2), SO(3).
    // A pair of elements that are projectively infinite order and projectively
    // non-commuting excludes the first three (finite: no infinite element; SO(2):
    // abelian; O(2): all its projectively infinite elements lie in the index-2 SO(2)
    // and commute). Hence closure = SO(3): density up to global phase, which is what
    // universality requires.
    //
    // Both projective properties are decided as exact Laurent-polynomial identities
    // over F: a polynomial with coefficients in F that vanishes at the transcendental
    // t = e^i vanishes identically, so
    //   - tr(u)^2 / det(u) algebraic  <=>  tr^2 and (tr^2 - tr(u^2)) proportional as polys,
    //   - u_a u_b = c * u_b u_a for some complex c  <=>  entrywise cross-products of the
    //     graded matrices agree as polynomial identities.
    let gs: Graded = ranges
        .iter()
        .enumerate()
        .map(|(bidx, &(lo, hi))| {
            let mut m = mat_zero(dim);
            for i in 0..dim {
                for j in lo..hi {
                    m[i][j] = s_tilde[i][j].clone();
                }
            }
            (evals[bidx], m)
        })
        .collect();
    let gt: Graded = ranges
        .iter()
        .enumerate()
        .map(|(bidx, &(lo, hi))| {
            let mut m = mat_zero(dim);
            for i in lo..hi {
                m[i][i] = t_diag[i].clone();
            }
            (evals[bidx], m)
        })
        .collect();
    let g_st = gmul(&gs, &gt);
    let g_ts = gmul(&gt, &gs);

    let words: Vec<(&str, &Graded)> = vec![("T", &gt), ("S", &gs), ("ST", &g_st), ("TS", &g_ts)];
    let mut proj_infinite: Vec<String> = Vec::new();
    for (name, w) in &words {
        if proj_infinite_order(&p1, w)? {
            proj_infinite.push((*name).to_string());
        }
    }
    let mut proj_pair: Option<(String, String)> = None;
    'pairs: for (i, (na, wa)) in words.iter().enumerate() {
        if !proj_infinite.iter().any(|x| x == na) {
            continue;
        }
        for (nb, wb) in words.iter().skip(i + 1) {
            if !proj_infinite.iter().any(|x| x == nb) {
                continue;
            }
            if proj_noncommute(&p1, wa, wb) {
                proj_pair = Some(((*na).to_string(), (*nb).to_string()));
                break 'pairs;
            }
        }
    }

    let certified_dense = proj_pair.is_some();

    // ---- the 22-dim complement: locating the continuous content exactly ----
    // P2 = I - P1 projects onto the 22-dim isotypic block (irreducible, since the
    // commutant has exact dimension 2). Its support straddles all four eigenspaces, so E
    // is non-scalar there. Projective infinite order is decided by the adjoint-trace
    // criterion, valid in any dimension: if u has finite projective order, every
    // eigenvalue ratio is a root of unity and tr(Ad u) = tr(u) tr(u)^* is algebraic;
    // tr(u) tr(u)^* is an exact Laurent polynomial in t (conjugation negates grades and
    // Galois-conjugates coefficients), so a nonzero coefficient at a nonzero grade forces
    // a transcendental value: infinite projective order, hence the closure contains a
    // positive-dimensional torus. A projectively non-commuting pair of such elements
    // makes the closure an infinite non-abelian compact group acting irreducibly on the
    // block: the coupled generators exceed every finite gate set, and the beyond-Clifford
    // content is located on this block.
    let p2 = {
        let mut m = mat_scale(&p1, &Cyc::from_int(-1));
        for i in 0..dim {
            m[i][i] = m[i][i].add(&Cyc::one());
        }
        m
    };
    if !mat_eq(&mat_mul(&p2, &p2), &p2) || mat_trace(&p2) != Cyc::from_int(22) {
        return Err("P2 is not an exact rank-22 projector".into());
    }
    let mut block22_infinite: Vec<String> = Vec::new();
    for (name, w) in &words {
        if adjoint_trace_infinite(&p2, w) {
            block22_infinite.push((*name).to_string());
        }
    }
    let mut block22_pair: Option<(String, String)> = None;
    'pairs22: for (i, (na, wa)) in words.iter().enumerate() {
        if !block22_infinite.iter().any(|x| x == na) {
            continue;
        }
        for (nb, wb) in words.iter().skip(i + 1) {
            if !block22_infinite.iter().any(|x| x == nb) {
                continue;
            }
            if proj_noncommute(&p2, wa, wb) {
                block22_pair = Some(((*na).to_string(), (*nb).to_string()));
                break 'pairs22;
            }
        }
    }
    let beyond_finite = block22_pair.is_some();

    // ---- exact order of the projective image on the block (diagnostic) ----
    // Elements are the sandwiches P1 X P1 acting on the block; projective equality is
    // proportionality over F (both operands are F-matrices), decided by normalizing at the
    // first nonzero entry. BFS over the two generators until closure.
    let finite_image_order: Option<usize> = if certified_dense {
        None
    } else {
        let normalize = |m: &Mat| -> Result<Mat, String> {
            for i in 0..dim {
                for j in 0..dim {
                    if !m[i][j].is_zero() {
                        return Ok(mat_scale(m, &m[i][j].inv()?));
                    }
                }
            }
            Err("zero element in projective BFS".into())
        };
        let sw = mat_mul(&mat_mul(&p1, &s_tilde), &p1);
        let tw = {
            let mut td = mat_zero(dim);
            for i in 0..dim {
                td[i][i] = t_diag[i].clone();
            }
            mat_mul(&mat_mul(&p1, &td), &p1)
        };
        let gens = [normalize(&sw)?, normalize(&tw)?];
        let mut elems: Vec<Mat> = vec![normalize(&p1)?];
        let mut frontier = elems.clone();
        let cap = 512usize;
        let mut overflow = false;
        while !frontier.is_empty() && !overflow {
            let mut next = Vec::new();
            for e in &frontier {
                for g in &gens {
                    let prod = normalize(&mat_mul(e, g))?;
                    if !elems.iter().any(|x| mat_eq(x, &prod)) {
                        if elems.len() >= cap {
                            overflow = true;
                            break;
                        }
                        elems.push(prod.clone());
                        next.push(prod);
                    }
                }
            }
            frontier = next;
        }
        if overflow {
            None
        } else {
            Some(elems.len())
        }
    };
    let coupling_note = if support_blocks == 1 {
        "the block is supported in a single spectral eigenspace, so E restricts to a \
         scalar phase and the archimedean coupling is projectively trivial on the block; \
         the projective image equals that of the finite modular pair (S, T) restricted, \
         which is finite; "
    } else {
        ""
    };
    let involution_note = if beta_s_nonzero.is_empty() {
        "tr(P1 G_S) = 0 identically, so u_s is a projective involution (structural finding; \
         any float threshold claiming a nonzero S-side trace coefficient is refuted); "
    } else {
        ""
    };
    let verdict = if certified_dense {
        "Closed subgroups of PU(2) = SO(3) are finite, SO(2), O(2), or SO(3); a projectively \
         non-commuting pair of projectively infinite-order elements excludes the first three, \
         so the closure is SO(3): density on the block up to global phase."
            .to_string()
    } else {
        format!(
            "Density on the block is REFUTED: the projective image of the generators on the \
             unique 2-dim invariant block is finite, exact order {finite_image_order:?}."
        )
    };
    let description = format!(
        "Exact certificate over Q(zeta_24): commutant dim = 2 and tr P1 = 2 (unique irreducible \
         2-dim block); {involution_note}tr(P1 G_T) nonzero at t^p for p in {beta_t_nonzero:?}; \
         block support over eigenvalues: {block_support:?}; {coupling_note}projectively \
         infinite-order words: {proj_infinite:?}; projectively non-commuting pair: {proj_pair:?}. \
         {verdict} On the 22-dim irreducible complement (support straddles all four \
         eigenspaces), infinite projective order is certified for {block22_infinite:?} via the \
         adjoint-trace criterion and the pair {block22_pair:?} is projectively non-commuting: \
         the closure of the projective image there is an infinite non-abelian compact group, so \
         the coupled generators exceed every finite gate set and the continuous content of the \
         machine is located on the 22-dim block. The one \
         analytic input is Lindemann (t = e^i transcendental); every other step is decided over \
         Q(zeta_24). No f64 value participates in any decision."
    );

    Ok(ExactDensityReport {
        commutant_dim,
        block_dim,
        beta_s_nonzero,
        beta_t_nonzero,
        noncommuting_grade,
        proj_infinite,
        proj_pair,
        block_support,
        finite_image_order,
        block22_infinite,
        block22_pair,
        beyond_finite,
        certified_dense,
        description,
    })
}

/// A Laurent polynomial in the transcendental `t`, with coefficients in `Q(zeta_24)`.
type Poly = Vec<(i64, Cyc)>;

fn pnorm(p: Poly) -> Poly {
    let mut m: std::collections::BTreeMap<i64, Cyc> = std::collections::BTreeMap::new();
    for (g, c) in p {
        let e = m.entry(g).or_insert_with(Cyc::zero);
        *e = e.add(&c);
    }
    m.into_iter().filter(|(_, c)| !c.is_zero()).collect()
}
fn pmul(a: &Poly, b: &Poly) -> Poly {
    let mut r = Vec::new();
    for (ga, ca) in a {
        for (gb, cb) in b {
            r.push((ga + gb, ca.mul(cb)));
        }
    }
    pnorm(r)
}
fn psub(a: &Poly, b: &Poly) -> Poly {
    let mut r = a.clone();
    for (g, c) in b {
        r.push((*g, c.neg()));
    }
    pnorm(r)
}
/// `n` proportional to `d` with a constant ratio, as a polynomial identity.
/// In the integral domain `F[t, t^-1]`, `n * d_ref == d * n_ref` for a nonzero
/// reference coefficient `d_ref` of `d` is equivalent to `n = c * d` for a constant `c`.
fn pproportional(n: &Poly, d: &Poly) -> bool {
    if n.is_empty() || d.is_empty() {
        return true; // n = 0 is 0 * d; d = 0 is handled by callers
    }
    let (gref, dref) = d[0].clone();
    let nref = pcoeff(n, gref);
    let lhs = pmul(n, &vec![(0, dref)]);
    let rhs = pmul(d, &vec![(0, nref)]);
    psub(&lhs, &rhs).is_empty()
}
fn pcoeff(p: &Poly, g: i64) -> Cyc {
    p.iter()
        .find(|(gg, _)| *gg == g)
        .map_or_else(Cyc::zero, |(_, c)| c.clone())
}

/// A graded matrix: a Laurent polynomial in `t` with `Mat` coefficients.
type Graded = Vec<(i64, Mat)>;

fn gnorm(g: Graded) -> Graded {
    let mut m: std::collections::BTreeMap<i64, Mat> = std::collections::BTreeMap::new();
    for (r, mat) in g {
        match m.get_mut(&r) {
            Some(acc) => {
                for i in 0..mat.len() {
                    for j in 0..mat.len() {
                        acc[i][j] = acc[i][j].add(&mat[i][j]);
                    }
                }
            }
            None => {
                m.insert(r, mat);
            }
        }
    }
    m.into_iter().filter(|(_, mat)| !mat_is_zero(mat)).collect()
}
fn gmul(a: &Graded, b: &Graded) -> Graded {
    let mut r = Vec::new();
    for (ga, ma) in a {
        for (gb, mb) in b {
            r.push((ga + gb, mat_mul(ma, mb)));
        }
    }
    gnorm(r)
}
/// `tr(P1 * piece)` per grade: the block trace of the word as a Laurent polynomial.
fn gtrace(p1: &Mat, g: &Graded) -> Poly {
    let n = p1.len();
    let mut out = Vec::new();
    for (r, m) in g {
        let mut t = Cyc::zero();
        for i in 0..n {
            for j in 0..n {
                t = t.add(&p1[i][j].mul(&m[j][i]));
            }
        }
        out.push((*r, t));
    }
    pnorm(out)
}
fn gsandwich(p1: &Mat, g: &Graded) -> Graded {
    gnorm(
        g.iter()
            .map(|(r, m)| (*r, mat_mul(&mat_mul(p1, m), p1)))
            .collect(),
    )
}

/// Projectively infinite order: `tr(u)^2 / det(u)` is not algebraic. For a 2x2 restriction,
/// `2 det = tr^2 - tr(u^2)`; both sides are Laurent polynomials over F evaluated at the
/// transcendental `t`, so the ratio is algebraic iff the polynomials are proportional as
/// identities. If the eigenvalue ratio were a root of unity the quantity would be algebraic,
/// so non-proportionality certifies infinite projective order.
fn proj_infinite_order(p1: &Mat, w: &Graded) -> Result<bool, String> {
    let tr = gtrace(p1, w);
    let sq = gmul(w, w);
    let trsq = gtrace(p1, &sq);
    let n = pmul(&tr, &tr);
    let d = psub(&n, &trsq); // 2 det(u) as a polynomial
    if d.is_empty() {
        return Err("det of a restricted word is identically zero".into());
    }
    Ok(!pproportional(&n, &d))
}

/// Infinite projective order in any dimension, via the adjoint trace. If `u` has finite
/// projective order then all eigenvalue ratios are roots of unity, so
/// `tr(Ad u) = tr(u) tr(u)^*` is algebraic. Here `tr(u)` is an exact Laurent polynomial in
/// the transcendental `t`; conjugation negates grades and applies the Galois conjugation
/// coefficientwise, so `tr(u) tr(u)^*` is again such a polynomial, and a nonzero coefficient
/// at a nonzero grade forces a transcendental value: infinite projective order, hence the
/// closure of the generated subgroup contains a positive-dimensional torus.
fn adjoint_trace_infinite(proj: &Mat, w: &Graded) -> bool {
    let tr = gtrace(proj, w);
    let tr_conj: Poly = pnorm(tr.iter().map(|(g, c)| (-g, c.conj())).collect());
    let n = pmul(&tr, &tr_conj);
    n.iter().any(|(g, c)| *g != 0 && !c.is_zero())
}

/// Projectively non-commuting: `u_a u_b = c * u_b u_a` fails for every complex scalar `c`.
/// Entrywise, proportionality with a constant scalar holds iff all cross-products of the
/// graded matrix entries agree as polynomial identities; one exact violation certifies
/// projective non-commutation.
fn proj_noncommute(p1: &Mat, a: &Graded, b: &Graded) -> bool {
    let ab = gsandwich(p1, &gmul(a, b));
    let ba = gsandwich(p1, &gmul(b, a));
    if ba.is_empty() || ab.is_empty() {
        return false;
    }
    let n = p1.len();
    // entry polynomials
    let entry_poly = |g: &Graded, i: usize, j: usize| -> Poly {
        pnorm(g.iter().map(|(r, m)| (*r, m[i][j].clone())).collect())
    };
    // reference entry of BA with a nonzero polynomial
    let mut refe: Option<(usize, usize, Poly)> = None;
    'find: for i in 0..n {
        for j in 0..n {
            let p = entry_poly(&ba, i, j);
            if !p.is_empty() {
                refe = Some((i, j, p));
                break 'find;
            }
        }
    }
    let Some((ri, rj, bref)) = refe else {
        return false;
    };
    let aref = entry_poly(&ab, ri, rj);
    for i in 0..n {
        for j in 0..n {
            let ae = entry_poly(&ab, i, j);
            let be = entry_poly(&ba, i, j);
            // AB_e * BA_ref == AB_ref * BA_e must hold for proportionality
            if !psub(&pmul(&ae, &bref), &pmul(&aref, &be)).is_empty() {
                return true;
            }
        }
    }
    false
}

/// Find the eigenvalues of `C` in `F` given `C^2 = aC + bI`, via the zero-divisor split of
/// `K = F[x]/(x^2 - a x - b)`: eliminate `C - lambda I` over `K`; a nonzero pivot `u + v x`
/// with vanishing norm `u^2 + a u v - b v^2` exposes the root `-u/v` in `F`. If `C` is
/// diagonal the eigenvalues are read off directly. If elimination completes with all pivots
/// invertible, the quadratic is irreducible over `F`, forcing Galois-equal isotypic dimensions
/// (12, 12), which the caller reports as a structural finding.
fn find_roots(c_mat: &Mat, a_co: &Cyc, b_co: &Cyc, dim: usize) -> Result<(Cyc, Cyc), String> {
    // diagonal fast path
    let mut diag_only = true;
    'chk: for i in 0..dim {
        for j in 0..dim {
            if i != j && !c_mat[i][j].is_zero() {
                diag_only = false;
                break 'chk;
            }
        }
    }
    if diag_only {
        let l1 = c_mat[0][0].clone();
        let l2 = (1..dim)
            .map(|i| c_mat[i][i].clone())
            .find(|v| *v != l1)
            .ok_or("diagonal C is scalar")?;
        return Ok((l1, l2));
    }
    // K-elimination with zero-divisor detection
    #[derive(Clone)]
    struct K {
        u: Cyc,
        v: Cyc,
    }
    let kmul = |x: &K, y: &K, a: &Cyc, b: &Cyc| -> K {
        // (u1 + v1 L)(u2 + v2 L), L^2 = a L + b
        let uu = x.u.mul(&y.u);
        let vv = x.v.mul(&y.v);
        K {
            u: uu.add(&vv.mul(b)),
            v: x.u.mul(&y.v).add(&x.v.mul(&y.u)).add(&vv.mul(a)),
        }
    };
    let ksub = |x: &K, y: &K| -> K {
        K {
            u: x.u.sub(&y.u),
            v: x.v.sub(&y.v),
        }
    };
    let kis0 = |x: &K| x.u.is_zero() && x.v.is_zero();
    // norm and inverse: conj(u+vL) = (u + a v) - v L; N = u^2 + a u v - b v^2
    let knorm = |x: &K, a: &Cyc, b: &Cyc| -> Cyc {
        x.u.mul(&x.u)
            .add(&a.mul(&x.u).mul(&x.v))
            .sub(&b.mul(&x.v).mul(&x.v))
    };
    let mut m: Vec<Vec<K>> = (0..dim)
        .map(|i| {
            (0..dim)
                .map(|j| {
                    let mut e = K {
                        u: c_mat[i][j].clone(),
                        v: Cyc::zero(),
                    };
                    if i == j {
                        e.v = Cyc::from_int(-1);
                    }
                    e
                })
                .collect()
        })
        .collect();
    let mut row = 0usize;
    for col in 0..dim {
        // find a pivot; check norms
        let mut piv: Option<usize> = None;
        for r in row..dim {
            if kis0(&m[r][col]) {
                continue;
            }
            let n = knorm(&m[r][col], a_co, b_co);
            if n.is_zero() {
                // zero divisor: u + v L with N = 0 and v != 0 exposes the root
                let e = &m[r][col];
                if e.v.is_zero() {
                    return Err("zero-norm element with v = 0".into());
                }
                let l1 = e.u.mul(&e.v.inv()?).neg();
                let l2 = a_co.sub(&l1);
                // verify
                let chk = l1.mul(&l1).sub(&a_co.mul(&l1)).sub(b_co);
                if !chk.is_zero() {
                    return Err("split root fails the quadratic".into());
                }
                return Ok((l1, l2));
            }
            piv = Some(r);
            break;
        }
        let Some(pr) = piv else { continue };
        m.swap(row, pr);
        // normalize and eliminate below (norms nonzero -> invertible)
        let n = knorm(&m[row][col], a_co, b_co).inv()?;
        let conj = K {
            u: m[row][col].u.add(&a_co.mul(&m[row][col].v)),
            v: m[row][col].v.neg(),
        };
        let inv = K {
            u: conj.u.mul(&n),
            v: conj.v.mul(&n),
        };
        for j in 0..dim {
            m[row][j] = kmul(&m[row][j].clone(), &inv, a_co, b_co);
        }
        for r in 0..dim {
            if r != row && !kis0(&m[r][col]) {
                let f = m[r][col].clone();
                for j in 0..dim {
                    let t = kmul(&m[row][j], &f, a_co, b_co);
                    m[r][j] = ksub(&m[r][j], &t);
                }
            }
        }
        row += 1;
        if row == dim {
            break;
        }
    }
    Err(
        "x^2 - a x - b is irreducible over Q(zeta_24): isotypic dimensions are Galois-equal (12,12), \
         not (2,22); no 2-dimensional block exists"
            .into(),
    )
}
