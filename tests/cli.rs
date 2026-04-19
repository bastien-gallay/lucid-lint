//! CLI integration tests.
//!
//! Exercise the `lucid-lint` binary via `assert_cmd` using fixture files
//! from `tests/corpus/`.

use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn corpus_path(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("corpus");
    path.push(relative);
    path
}

#[test]
fn check_reports_long_sentence_in_english_sample() {
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("public")
        .arg(fixture)
        .assert()
        .code(1) // warnings present
        .stdout(predicate::str::contains("sentence-too-long"))
        .stdout(predicate::str::contains("Summary:"));
}

#[test]
fn check_reports_long_sentence_in_french_sample() {
    let fixture = corpus_path("fr/echantillon.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("public")
        .arg(fixture)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("sentence-too-long"));
}

#[test]
fn check_with_json_format_produces_valid_json() {
    let fixture = corpus_path("en/sample.md");
    let output = Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--format")
        .arg("json")
        .arg(fixture)
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed["version"], 1);
    assert!(parsed["diagnostics"].is_array());
    assert!(parsed["summary"]["total"].as_u64().unwrap() >= 1);
}

#[test]
fn check_dev_doc_profile_passes_sample() {
    // The English sample's long sentence is under the DevDoc threshold of 30.
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("dev-doc")
        .arg(fixture)
        .assert()
        .success();
}

#[test]
fn check_falc_profile_is_stricter() {
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("falc")
        .arg(fixture)
        .assert()
        .code(1);
}

#[test]
fn check_rejects_unknown_profile() {
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("nonexistent")
        .arg(fixture)
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn check_accepts_stdin() {
    let input = "This is a rather long sentence that keeps adding more and more words \
                 until it exceeds the public profile threshold by a comfortable margin.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("-")
        .write_stdin(input)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("sentence-too-long"));
}

#[test]
fn check_clean_text_returns_zero() {
    let input = "Short sentence. Another short one.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("-")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("No issues found"));
}

#[test]
fn check_requires_path_argument() {
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .assert()
        .failure();
}

#[test]
fn check_handles_directory_argument() {
    let dir = corpus_path("en");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg(dir)
        .assert()
        .code(1); // sample.md has warnings
}
