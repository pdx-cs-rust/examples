use rc::refcell::Counter;

#[test]
#[should_panic]
fn test_double_ref_fails() {
    let c = Counter::default();
    let _r1 = c.count_mut();
    // Fails here as the count is already loaned out.
    let _r2 = c.count_mut();
}
