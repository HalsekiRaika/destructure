#[test]
fn tests() {
    let try_test =  trybuild::TestCases::new();
    try_test.pass("tests/01-parse.rs");
    try_test.pass("tests/02-generate.rs");
}