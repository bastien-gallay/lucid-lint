//! Data-driven regression over the Fable-generated corpus.
//!
//! Walks `tests/corpus/generated/<lang>/<expect>/<rule>/*.md` and, for each
//! fixture, runs the CLI under the `falc` profile and asserts the path-encoded
//! expectation:
//!   - `fire`  → the target rule MUST be among the fired diagnostics,
//!   - `clean` → the target rule must NOT fire.
//!
//! The layout IS the specification: adding a fixture under the right directory
//! is enough to extend coverage — no per-file test to hand-wire. Fixtures are
//! promoted from the Fable harness staging (`.personal/fable-harness/`), each
//! already validated by this same oracle before landing here.
#![allow(clippy::panic)] // A broken fixture or unreadable dir must fail the test loudly, naming the culprit.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assert_cmd::prelude::*;

fn generated_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/generated")
}

/// Rule ids fired by `fixture` under `profile` (mirrors the helper in cli.rs).
fn rule_ids_fired(fixture: &Path, profile: &str) -> Vec<String> {
    let output = Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg(profile)
        .arg("--format")
        .arg("json")
        .arg(fixture)
        .output()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
        panic!(
            "lucid-lint did not emit JSON for {}: {err}\nstdout: {}\nstderr: {}",
            fixture.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        )
    });
    parsed["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["rule_id"].as_str().unwrap().to_string())
        .collect()
}

/// Collect every `.md` under `dir`, recursively.
fn walk_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(dir).unwrap_or_else(|err| panic!("cannot read {}: {err}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|err| panic!("cannot read an entry under {}: {err}", dir.display()))
            .path();
        if path.is_dir() {
            walk_md(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

#[test]
fn generated_corpus_matches_path_encoded_expectation() {
    let root = generated_root();
    if !root.exists() {
        return; // no generated corpus yet — nothing to assert
    }

    let mut fixtures = Vec::new();
    walk_md(&root, &mut fixtures);
    assert!(
        !fixtures.is_empty(),
        "tests/corpus/generated exists but holds no .md fixtures"
    );

    let mut failures = Vec::new();
    for path in &fixtures {
        // .../generated/<lang>/<expect>/<rule>/<id>.md
        let rel = path.strip_prefix(&root).unwrap();
        let parts: Vec<_> = rel
            .iter()
            .map(|c| c.to_string_lossy().into_owned())
            .collect();
        assert!(
            parts.len() >= 4,
            "unexpected layout for {}; want <lang>/<expect>/<rule>/<file>.md",
            rel.display()
        );
        let (lang, expect, rule_name) = (&parts[0], &parts[1], &parts[2]);
        assert!(
            matches!(lang.as_str(), "en" | "fr"),
            "bad lang segment in {}",
            rel.display()
        );
        assert!(
            matches!(expect.as_str(), "fire" | "clean"),
            "bad expect segment in {}",
            rel.display()
        );
        let rule_id = format!("lexicon.{rule_name}");

        let fired = rule_ids_fired(path, "falc");
        let hit = fired.iter().any(|r| r == &rule_id);
        let ok = if expect == "fire" { hit } else { !hit };
        if !ok {
            failures.push(format!(
                "  {} :: expected {expect} {rule_id}, fired {fired:?}",
                rel.display()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} generated fixture(s) drifted from expectation:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
