#[test]
fn test_construction() {
    let t = trybuild::TestCases::new();
    t.pass("tests/init.rs");
    // t.pass("tests/macro/structure.rs");
    // t.pass("tests/macro/decompose.rs");
    // t.pass("tests/macro/construct.rs");
}
