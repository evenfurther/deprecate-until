// Check exact error messages (only run when no_ui_output feature is NOT enabled)
#[cfg(not(feature = "no_ui_output"))]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

// On non-nightly, just verify compilation fails without checking error messages
#[cfg(feature = "no_ui_output")]
#[test]
fn ui_compile_fail_only() {
    use std::path::Path;
    use std::process::Command;

    let ui_dir = Path::new("tests/ui");

    for entry in std::fs::read_dir(ui_dir)
        .expect("Failed to read ui directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
    {
        let path = entry.path();

        let output = Command::new("rustc")
            .arg("--crate-type=lib")
            .arg(&path)
            .arg("-o")
            .arg("/dev/null")
            .output()
            .expect("Failed to run rustc");

        assert!(
            !output.status.success(),
            "Expected {} to fail compilation, but it succeeded",
            path.display()
        );
    }
}
