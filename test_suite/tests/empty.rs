#[test]
fn test_construction() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pkl_derive/init.rs");
    t.pass("tests/pkl_derive/simple.rs");
    // t.pass("tests/macro/decompose.rs");
    // t.pass("tests/macro/construct.rs");
}
