//! CLI integration tests.
//!
//! Exercise the `lucid-lint` binary via `assert_cmd` using fixture files
//! from `tests/corpus/`.

use std::fs;
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
        .stdout(predicate::str::contains("structure.sentence-too-long"))
        .stdout(predicate::str::contains("summary:"));
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
        .stdout(predicate::str::contains("structure.sentence-too-long"));
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
    assert_eq!(parsed["version"], 2);
    assert!(parsed["diagnostics"].is_array());
    assert!(parsed["summary"]["total"].as_u64().unwrap() >= 1);
    assert_eq!(parsed["score"]["max"], 100);
    assert_eq!(parsed["category_scores"].as_array().unwrap().len(), 5);
    assert!(parsed["diagnostics"][0]["weight"].as_u64().is_some());
}

#[test]
fn check_min_score_gate_trips_on_clean_text() {
    // Clean text emits only an info `readability-score` diagnostic, so the
    // severity gate passes. With `--min-score=100` the score gate alone must
    // flip the exit code when the score is below 100.
    let input = "Short sentence. Another short one.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--min-score")
        .arg("100")
        .arg("-")
        .write_stdin(input)
        .assert()
        .code(1);
}

#[test]
fn check_min_score_zero_always_passes_gate() {
    let input = "Short sentence. Another short one.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--min-score")
        .arg("0")
        .arg("-")
        .write_stdin(input)
        .assert()
        .success();
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
fn check_accepts_conditions_flag() {
    // `--conditions` is parsed; with the v0.2 rule set (all `general`)
    // it does not change behavior. Smoke test that the flag is wired and
    // that comma-separated values parse.
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--profile")
        .arg("public")
        .arg("--conditions")
        .arg("dyslexia,aphasia")
        .arg(fixture)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("structure.sentence-too-long"));
}

#[test]
fn check_rejects_unknown_condition_tag() {
    let fixture = corpus_path("en/sample.md");
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--conditions")
        .arg("autism")
        .arg(fixture)
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
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
        .arg("--profile")
        .arg("public")
        .arg("-")
        .write_stdin(input)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("structure.sentence-too-long"));
}

#[test]
fn check_clean_text_returns_zero() {
    // `readability-score` emits an observability `info` diagnostic on every
    // non-empty document, so clean text exits 0 with an info-only summary
    // rather than "No issues found".
    let input = "Short sentence. Another short one.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("-")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("summary: 1 info"));
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
        .arg("--profile")
        .arg("public")
        .arg(dir)
        .assert()
        .code(1); // sample.md has warnings
}

/// F19 — `[[ignore]]` in `lucid-lint.toml` silences matching rule ids
/// on stdin (which has no inline-disable escape hatch).
#[test]
fn check_config_ignore_silences_rule_on_stdin() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("lucid-lint.toml");
    fs::write(
        &config,
        r#"[[ignore]]
rule_id = "structure.sentence-too-long"
"#,
    )
    .unwrap();

    let input = "This is a rather long sentence that keeps adding more and more words \
                 until it exceeds the public profile threshold by a comfortable margin.";

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg("-")
        .write_stdin(input)
        .assert()
        .stdout(predicate::str::contains("structure.sentence-too-long").not());
}

/// F19 — `[[ignore]]` also applies to Markdown files, giving users a
/// global alternative to inline directives when silencing a rule
/// everywhere at once is less noisy than sprinkling comments.
#[test]
fn check_config_ignore_silences_rule_in_markdown() {
    let tmp = tempfile::tempdir().unwrap();
    let doc = tmp.path().join("doc.md");
    fs::write(
        &doc,
        "This is a rather long sentence that keeps adding more and more words \
         until it exceeds the public profile threshold by a comfortable margin.\n",
    )
    .unwrap();

    let config = tmp.path().join("lucid-lint.toml");
    fs::write(
        &config,
        r#"[[ignore]]
rule_id = "structure.sentence-too-long"
"#,
    )
    .unwrap();

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg(&doc)
        .assert()
        .stdout(predicate::str::contains("structure.sentence-too-long").not());
}

/// F19 — unknown rule ids in the config are tolerated silently
/// (matches the `[scoring.weights]` precedent for future-compat).
#[test]
fn check_config_ignore_tolerates_unknown_rule_id() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("lucid-lint.toml");
    fs::write(
        &config,
        r#"[[ignore]]
rule_id = "rule-that-does-not-exist"
"#,
    )
    .unwrap();

    let input = "Short sentence. Another short one.";
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg("-")
        .write_stdin(input)
        .assert()
        .success();
}

/// F78 — `--exclude` prunes files during directory recursion.
///
/// Fixture: a temp dir with two warnings-generating files. Excluding
/// one by glob must leave the other discovered, and diagnostics must
/// reference only the retained file.
#[test]
fn check_exclude_flag_prunes_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let noisy = "This is a deliberately long sentence that rambles on and on without any \
                 clear purpose other than crossing the public-profile word budget so it \
                 trips the sentence-too-long rule reliably in any locale.";

    fs::write(root.join("keep.md"), noisy).unwrap();
    fs::create_dir_all(root.join("vendor")).unwrap();
    fs::write(root.join("vendor/skip.md"), noisy).unwrap();

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--exclude")
        .arg("vendor/**")
        .arg(root)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("keep.md"))
        .stdout(predicate::str::contains("vendor/skip.md").not());
}

/// F78 — `exclude = [...]` in `lucid-lint.toml` prunes the same way as
/// the CLI flag. Covers the auto-discovery path: the config is written
/// next to the files and found via `--config`.
#[test]
fn check_exclude_from_toml_config_prunes_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let noisy = "This is a deliberately long sentence that rambles on and on without any \
                 clear purpose other than crossing the public-profile word budget so it \
                 trips the sentence-too-long rule reliably in any locale.";

    fs::write(root.join("keep.md"), noisy).unwrap();
    fs::create_dir_all(root.join("fixtures")).unwrap();
    fs::write(root.join("fixtures/skip.md"), noisy).unwrap();

    let config = root.join("lucid-lint.toml");
    fs::write(&config, "[default]\nexclude = [\"fixtures/**\"]\n").unwrap();

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg(root)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("keep.md"))
        .stdout(predicate::str::contains("fixtures/skip.md").not());
}

/// F78 — explicit file arguments bypass exclusion. If the user names a
/// path directly, the glob list does not apply.
#[test]
fn check_exclude_does_not_filter_explicit_file_args() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("vendor.md");

    let noisy = "This is a deliberately long sentence that rambles on and on without any \
                 clear purpose other than crossing the public-profile word budget so it \
                 trips the sentence-too-long rule reliably in any locale.";
    fs::write(&file, noisy).unwrap();

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--exclude")
        .arg("**/vendor.md")
        .arg(&file)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("vendor.md"));
}

/// `[rules."structure.excessive-commas"].max_commas` lowers the
/// threshold below the Public profile default (3). A sentence with
/// exactly 3 subordination commas passes under Public defaults but
/// trips once the config narrows `max_commas` to 2.
#[test]
fn check_config_excessive_commas_override_tightens_threshold() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("lucid-lint.toml");
    fs::write(
        &config,
        r#"[rules."structure.excessive-commas"]
max_commas = 2
"#,
    )
    .unwrap();

    // 3 subordination commas: passes Public default (3), fails override (2).
    let input = "Note, although we agreed, we packed carefully, and quietly.";

    // Baseline: without the config, the sentence is clean under Public.
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("-")
        .write_stdin(input)
        .assert()
        .stdout(predicate::str::contains("structure.excessive-commas").not());

    // With the config, the same sentence trips the rule.
    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg("-")
        .write_stdin(input)
        .assert()
        .stdout(predicate::str::contains("structure.excessive-commas"));
}

/// Invalid `max_commas` values (zero, negative, non-integer) must
/// surface as a parse error rather than silently falling back.
#[test]
fn check_config_excessive_commas_rejects_zero() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("lucid-lint.toml");
    fs::write(
        &config,
        r#"[rules."structure.excessive-commas"]
max_commas = 0
"#,
    )
    .unwrap();

    Command::cargo_bin("lucid-lint")
        .unwrap()
        .arg("check")
        .arg("--config")
        .arg(&config)
        .arg("-")
        .write_stdin("anything")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("max_commas"));
}

// ---------------------------------------------------------------
// Real-world corpus regression anchors
//
// Short passages lifted verbatim from `examples/public/` — curated
// plain-language prose and deliberately-dense legalese. Each test
// pins a specific rule-level expectation so a rule-tuning change
// that quietly alters real-world behaviour fails loudly in CI.
// ---------------------------------------------------------------

fn rule_ids_fired(fixture: &std::path::Path, profile: &str) -> Vec<String> {
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
    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    parsed["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["rule_id"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn corpus_public_gov_uk_prose_stays_clean_under_public_profile() {
    // GOV.UK's "Meet the user need" passage is a curated plain-language
    // exemplar. Under the `public` profile it must not trigger
    // sentence-length, weasel-word, or passive-voice diagnostics — these
    // would indicate a rule regression, not a problem in the source.
    let fired = rule_ids_fired(
        &corpus_path("public/en/gov-uk-meet-the-user-need.md"),
        "public",
    );
    assert!(
        !fired.contains(&"structure.sentence-too-long".to_string()),
        "sentence-too-long regression on GOV.UK plain-prose anchor: {fired:?}"
    );
    assert!(
        !fired.contains(&"lexicon.weasel-words".to_string()),
        "weasel-words regression on GOV.UK plain-prose anchor: {fired:?}"
    );
    assert!(
        !fired.contains(&"syntax.passive-voice".to_string()),
        "passive-voice regression on GOV.UK plain-prose anchor: {fired:?}"
    );
}

#[test]
fn corpus_public_plainlanguage_intro_flags_dense_prose() {
    // plainlanguage.gov's own intro has two deliberately long opening
    // sentences. The `public` profile must flag `structure.sentence-too-long`;
    // losing this signal would mean the sentence-length detector silently
    // stopped catching a known-dense real-world passage.
    let fired = rule_ids_fired(
        &corpus_path("public/en/plainlanguage-gov-intro.md"),
        "public",
    );
    assert!(
        fired.iter().any(|r| r == "structure.sentence-too-long"),
        "expected sentence-too-long on plainlanguage.gov intro: {fired:?}"
    );

    // Under `falc` (stricter) passive-voice must also fire — the Plain
    // Writing Act sentence "content for the public is written for its
    // specific audience" is a canonical passive construction.
    let fired_falc = rule_ids_fired(&corpus_path("public/en/plainlanguage-gov-intro.md"), "falc");
    assert!(
        fired_falc.iter().any(|r| r == "syntax.passive-voice"),
        "expected passive-voice under falc: {fired_falc:?}"
    );
}

#[test]
fn corpus_public_vikidia_fr_stays_clean_under_falc() {
    // Vikidia targets 8–13-year-olds; the "Accueil" passage should pass
    // even the strictest `falc` profile without any `structure.sentence-too-long`
    // diagnostic. This is the tightest FR regression anchor we can pin.
    let fired = rule_ids_fired(&corpus_path("public/fr/vikidia-accueil.md"), "falc");
    assert!(
        !fired.contains(&"structure.sentence-too-long".to_string()),
        "sentence-too-long regression on Vikidia FR anchor under falc: {fired:?}"
    );
}
