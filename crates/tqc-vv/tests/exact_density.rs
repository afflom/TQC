use tqc_core::UseCaseParams;
use tqc_vv::exact;

/// Exact algebraic density certificate at the atlas use-case (modality 3, context 8).
/// Every decision is made over Q(zeta_24); no f64 value participates in the verdict.
#[test]
fn exact_density_atlas() {
    let p = UseCaseParams::new(4, 3, 8);
    let report = exact::exact_density_certificate(&p).expect("exact certificate must run");
    println!("commutant_dim      = {}", report.commutant_dim);
    println!("block_dim          = {}", report.block_dim);
    println!("beta_s nonzero at  = {:?}", report.beta_s_nonzero);
    println!("beta_t nonzero at  = {:?}", report.beta_t_nonzero);
    println!("noncommuting grade = {:?}", report.noncommuting_grade);
    println!("proj infinite      = {:?}", report.proj_infinite);
    println!("proj pair          = {:?}", report.proj_pair);
    println!("block support      = {:?}", report.block_support);
    println!("finite image order = {:?}", report.finite_image_order);
    println!("block22 infinite   = {:?}", report.block22_infinite);
    println!("block22 pair       = {:?}", report.block22_pair);
    println!("beyond finite      = {}", report.beyond_finite);
    println!("certified_dense    = {}", report.certified_dense);
    println!("{}", report.description);
    assert_eq!(report.commutant_dim, 2, "exact commutant dimension");
    assert_eq!(report.block_dim, 2, "exact block dimension");
    // Kernel-grade findings at the atlas use-case: the unique 2-dim invariant block lies
    // inside the (-1) eigenspace, the coupling is a global phase there, and the projective
    // image is finite. Density on the block is refuted, not certified.
    assert!(report.beta_s_nonzero.is_empty(), "tr(P1 G_S) = 0 identically");
    assert_eq!(report.beta_t_nonzero, vec![-1], "u_t trace grade");
    assert_eq!(
        report.block_support,
        vec![(10, 0.0), (7, 0.0), (2, 0.0), (-1, 2.0)],
        "block supported entirely in the (-1) eigenspace"
    );
    assert!(!report.certified_dense, "density on the block is refuted");
    assert!(report.finite_image_order.is_some(), "projective image is finite");
}
