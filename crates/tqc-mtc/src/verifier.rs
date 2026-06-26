use crate::{close_mat, identity, is_symmetric, is_unitary, mat_pow, zeros, Matrix, C};

/// Data for a generic Modular Tensor Category (MTC), potentially non-pointed.
pub trait ModularData {
    /// The number of simple objects.
    fn dim(&self) -> usize;
    /// The modular S matrix.
    fn s_matrix(&self) -> Matrix;
    /// The diagonal topological spins T.
    fn t_diag(&self) -> Vec<C>;
    /// The charge-conjugation permutation matrix.
    fn charge_conjugation(&self) -> Matrix;
    /// Fusion coefficient N_{ij}^k.
    fn n_ijk(&self, i: usize, j: usize, k: usize) -> f64;
    /// The F-symbol (associator).
    fn f_symbol(&self, i: usize, j: usize, k: usize, l: usize, m: usize, n: usize) -> C;
    /// The R-symbol (braiding).
    fn r_symbol(&self, i: usize, j: usize, k: usize) -> C;
}

/// Verify the universal MTC axioms for a generalized Atlas-native category.
///
/// **Implemented checks:**
/// - Modular `S` and `T` matrices satisfy SL(2,ℤ) relations (`S` symmetric/unitary, `S⁴ = I`, `(ST)³ = S²`, `S² = C`).
/// - Fusion coefficients `N_{ij}^k` are non-negative integers.
///
/// **Stubbed checks (represented only by trait signatures):**
/// - Full `F`-symbol pentagon coherence.
/// - Full hexagon coherence for `F` and `R`.
/// - Yang–Baxter coverage and monodromy consistency for general non-pointed MTCs.
/// - Full nonnegative integral Verlinde fusion checks linking `S` to `N_{ij}^k`.
///
/// *Crucial Distinction:* This generalized verifier does **not** check pentagon or hexagon coherence, because those checks are not yet implemented for an arbitrary (non-pointed) MTC.
/// By contrast, the pointed `D(Z_n)` representative (in `lib.rs`) implements its own specialized, exact checks for the hexagon, Yang-Baxter, and Verlinde formula that hold specifically because the category is pointed (bicharacter braiding).
/// The generalized verifier must never assert `some-true` or `build` for full pentagon/hexagon coherence until those checks are actually implemented.
///
/// # Errors
/// Returns a description of the first axiom that fails within `tol`.
pub fn verify_mtc_axioms<M: ModularData>(m: &M, tol: f64) -> Result<(), String> {
    let dim = m.dim();
    let s = m.s_matrix();
    let t = m.t_diag();

    if !is_symmetric(&s, tol) {
        return Err("S is not symmetric".into());
    }
    if !is_unitary(&s, tol) {
        return Err("S is not unitary".into());
    }

    for (i, theta) in t.iter().enumerate() {
        if (theta.abs2() - 1.0).abs() > tol {
            return Err(format!("T[{i}] is not a phase"));
        }
    }

    let s2 = mat_pow(&s, 2);
    if !close_mat(&mat_pow(&s, 4), &identity(dim), tol) {
        return Err("S^4 != I".into());
    }

    // (ST)^3 = S^2
    let mut st = zeros(dim);
    for (i, row) in st.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = s[i][j].times(t[j]);
        }
    }
    if !close_mat(&mat_pow(&st, 3), &s2, tol) {
        return Err("(ST)^3 != S^2".into());
    }

    if !close_mat(&s2, &m.charge_conjugation(), tol) {
        return Err("S^2 != charge conjugation".into());
    }

    // Fusion nonnegative integers
    for i in 0..dim {
        for j in 0..dim {
            for k in 0..dim {
                let n_val = m.n_ijk(i, j, k);
                if n_val < -tol || (n_val - n_val.round()).abs() > tol {
                    return Err(format!(
                        "N_{{{i},{j}}}^{k} is not a nonnegative integer: {n_val}"
                    ));
                }
            }
        }
    }

    // A complete implementation loops over objects and channels to verify pentagon and hexagon coherence.
    // We intentionally leave these checks unimplemented and return Ok(()) ONLY for the limited subset of axioms actually verified here (S/T relations and fusion non-negativity).
    // Do NOT assert full MTC coherence based on this verifier alone.
    Ok(())
}
