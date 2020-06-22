use lovm2::hir::*;

#[test]
fn building() {
    let mut hir = HIR::new();
    hir.branch();

    assert!(hir.build().is_ok());
}
