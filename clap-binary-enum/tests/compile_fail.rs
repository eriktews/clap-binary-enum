use std::path::PathBuf;
use std::process::Command;

#[test]
fn compile_fail() {
    let manifest_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let ui_dir = manifest_dir.join("tests/ui");

    let mut test_files: Vec<_> = std::fs::read_dir(&ui_dir)
        .unwrap()
        .filter_map(|e| {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|ext| ext == "rs") {
                Some(p)
            } else {
                None
            }
        })
        .collect();
    test_files.sort();

    for test_file in &test_files {
        let content = std::fs::read_to_string(test_file).unwrap();
        let file_name = test_file.file_name().unwrap().to_str().unwrap().to_string();
        let stem = file_name.trim_end_matches(".rs");

        let temp_dir = std::env::temp_dir().join(format!("clap-binary-enum-test-{stem}"));
        let _ = std::fs::remove_dir_all(&temp_dir);

        let src_dir = temp_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("main.rs"), &content).unwrap();

        let cargo_toml = format!(
            r#"[package]
name = "test-{stem}"
version = "0.1.0"
edition = "2021"

[dependencies]
clap-binary-enum = {{ path = "{}" }}
clap = {{ version = "4", features = ["derive"] }}
"#,
            manifest_dir.display()
        );
        std::fs::write(temp_dir.join("Cargo.toml"), &cargo_toml).unwrap();

        let output = Command::new("cargo")
            .arg("check")
            .current_dir(&temp_dir)
            .output()
            .unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            !output.status.success(),
            "Expected {file_name} to fail compilation, but it succeeded.\n{stderr}"
        );

        match stem {
            "not_an_enum" => {
                assert!(
                    stderr.contains("YesNoArg can only be used on enums"),
                    "Missing expected error in {file_name}:\n{stderr}"
                );
            }
            "wrong_variant_count" => {
                assert!(
                    stderr.contains("YesNoArg requires exactly two variants"),
                    "Missing expected error in {file_name}:\n{stderr}"
                );
            }
            "variants_with_fields" => {
                assert!(
                    stderr.contains("YesNoArg variants must have no fields"),
                    "Missing expected error in {file_name}:\n{stderr}"
                );
            }
            "unknown_enum_attr" => {
                assert!(
                    stderr.contains("unexpected yesno attribute; expected `name`"),
                    "Missing expected error in {file_name}:\n{stderr}"
                );
            }
            "unknown_variant_attr" => {
                assert!(
                    stderr.contains("unexpected yesno attribute; expected `help`"),
                    "Missing expected error in {file_name}:\n{stderr}"
                );
            }
            _ => panic!("Unknown test file: {file_name}"),
        }

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
