#[test]
fn test_version() {
    let out = cargo_script!("--version");
    assert!(out.success());
    scan!(&out.stdout;
        ("cargo-scripter", &::std::env::var("CARGO_PKG_VERSION").unwrap(), .._) => ()
    ).unwrap();
}
