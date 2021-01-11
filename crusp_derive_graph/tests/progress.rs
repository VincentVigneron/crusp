#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple-derive.rs");

}
