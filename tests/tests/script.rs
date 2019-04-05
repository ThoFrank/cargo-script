#[test]
fn test_script_explicit() {
    let out = cargo_script!("-dboolinator", "tests/data/script-explicit.rs");
    scan!(out.stdout_output();
        ("Some(1)") => ()
    ).unwrap()
}

#[test]
fn test_script_features() {
    let out = cargo_script!("--features", "dont-panic", "tests/data/script-features.rs");
    scan!(out.stdout_output();
        ("Keep calm and borrow check.") => ()
    ).unwrap();

    let out = cargo_script!("tests/data/script-features.rs");
    assert!(!out.success());
}

#[test]
fn test_script_full_block() {
    let out = cargo_script!("tests/data/script-full-block.rs");
    scan!(out.stdout_output();
        ("Some(1)") => ()
    ).unwrap()
}

#[test]
fn test_script_full_line() {
    let out = cargo_script!("tests/data/script-full-line.rs");
    scan!(out.stdout_output();
        ("Some(1)") => ()
    ).unwrap()
}

#[test]
fn test_script_invalid_doc_comment() {
    let out = cargo_script!("tests/data/script-invalid-doc-comment.rs");
    scan!(out.stdout_output();
        ("Hello, World!") => ()
    ).unwrap()
}

#[test]
fn test_script_no_deps() {
    let out = cargo_script!("tests/data/script-no-deps.rs");
    scan!(out.stdout_output();
        ("Hello, World!") => ()
    ).unwrap()
}

#[test]
fn test_script_short() {
    let out = cargo_script!("tests/data/script-short.rs");
    scan!(out.stdout_output();
        ("Some(1)") => ()
    ).unwrap()
}

#[test]
fn test_script_test() {
    let out = cargo_script!("--test", "tests/data/script-test.rs");
    assert!(out.success());
}

#[test]
fn test_script_hyphens() {
    use scan_rules::scanner::QuotedString;
    let out = cargo_script!("--", "tests/data/script-args.rs", "-NotAnArg");
    scan!(out.stdout_output();
        ("[0]:", let _: QuotedString, "[1]:", let arg: QuotedString) => {
            assert_eq!(arg, "-NotAnArg");
        }
    ).unwrap()
}

#[test]
fn test_script_has_weird_chars() {
    let out = cargo_script!("tests/data/script-has.weird§chars!.rs");
    assert!(out.success());
}

#[test]
fn test_script_slow_output() {
    let out = cargo_script!(
        "--use-shared-binary-cache=no",
        "tests/data/script-slow-output.rs"
    );
    assert!(out.stderr_raw().contains("Compiling slow-build"));
    assert!(out.stderr_raw().contains("Compiling script-slow-output"));
    scan!(out.stdout_output();
        ("Ok") => ()
    ).unwrap()
}

#[test]
fn test_script_cs_env() {
    let out = cargo_script!(
        "tests/data/script-cs-env.rs"
    );
    scan!(out.stdout_output();
        ("Ok") => ()
    ).unwrap()
}
