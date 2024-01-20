#[test]
fn tests() {
    let try_test =  trybuild::TestCases::new();
    try_test.pass("tests/01-parse.rs");
    try_test.pass("tests/02-generate.rs");
    try_test.pass("tests/03-generics.rs");
    try_test.pass("tests/04-freeze.rs");
    try_test.pass("tests/05-reconstructor.rs");
    try_test.pass("tests/06-try-reconstructor.rs");
    try_test.pass("tests/07-mutation.rs");
    try_test.pass("tests/08-try-mutation.rs");
    try_test.pass("tests/09-multiple-generics.rs");
}