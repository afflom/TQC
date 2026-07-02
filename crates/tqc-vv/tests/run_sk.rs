use tqc_core::UseCaseParams;
use tqc_vv::witness;
#[test]
fn run_sk_atlas() {
    let p = UseCaseParams::new(4, 3, 8); // atlas
    match witness::solovay_kitaev_probe(&p) {
        Ok(m) => println!("\n>>> is_dense={} :: {}", m.is_dense, m.description),
        Err(e) => println!("\n>>> Err: {e}"),
    }
}
