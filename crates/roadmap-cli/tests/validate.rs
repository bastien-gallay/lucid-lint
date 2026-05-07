//! Integration tests for the `validate` subcommand logic.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn unique_tmp(label: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("roadmap-cli-test-{label}-{pid}-{n}"))
}

fn render_minimal() -> String {
    let root = fixture("minimal");
    let config = roadmap_cli::load_config(&root).unwrap();
    let mut features = roadmap_cli::load_features(&root).unwrap();
    roadmap_cli::sort_features(&mut features, &config);
    roadmap_cli::render(&features)
}

#[test]
fn clean_run_against_matching_roadmap() {
    let root = fixture("minimal");
    let tmp = unique_tmp("clean");
    std::fs::create_dir_all(&tmp).unwrap();
    let roadmap_md = tmp.join("ROADMAP.md");
    std::fs::write(&roadmap_md, render_minimal()).unwrap();

    let report = roadmap_cli::validate::validate(&root, &roadmap_md).unwrap();
    assert!(
        report.is_clean(),
        "expected clean, got:\n{}",
        report.to_text()
    );
}

#[test]
fn drift_when_roadmap_lacks_an_anchor() {
    let root = fixture("minimal");
    let tmp = unique_tmp("missing-anchor");
    std::fs::create_dir_all(&tmp).unwrap();
    let roadmap_md = tmp.join("ROADMAP.md");
    // Write a stub that contains only one of the fixture's anchors.
    std::fs::write(&roadmap_md, r#"<a id="f22"></a>"#).unwrap();

    let report = roadmap_cli::validate::validate(&root, &roadmap_md).unwrap();
    assert!(report.has_drift());
    assert!(!report.has_hard_errors());
    assert!(report
        .anchors_missing_from_disk
        .contains(&"f-llm-plugin".to_string()));
    assert!(report
        .anchors_missing_from_disk
        .contains(&"f-roadmap-toml-source".to_string()));
}

#[test]
fn drift_when_roadmap_has_orphan_anchor() {
    let root = fixture("minimal");
    let tmp = unique_tmp("orphan-anchor");
    std::fs::create_dir_all(&tmp).unwrap();
    let roadmap_md = tmp.join("ROADMAP.md");
    let mut content = render_minimal();
    content.push_str("\n<a id=\"f-deleted-feature\"></a>\n");
    std::fs::write(&roadmap_md, content).unwrap();

    let report = roadmap_cli::validate::validate(&root, &roadmap_md).unwrap();
    assert!(report.has_drift());
    assert_eq!(
        report.anchors_missing_from_regen,
        vec!["f-deleted-feature".to_string()]
    );
    assert!(report.anchors_missing_from_disk.is_empty());
}

#[test]
fn schema_error_does_not_abort_run() {
    // Build a temp .roadmap/ with one valid + one broken feature file.
    let root = unique_tmp("schema-err");
    let features = root.join("features");
    std::fs::create_dir_all(&features).unwrap();
    std::fs::write(root.join("config.toml"), "versions = [\"v0.2.x\"]\n").unwrap();
    std::fs::write(
        features.join("f-good.md"),
        "+++\nid = \"F-good\"\ntopic = \"X\"\nstatus = \"todo\"\npriority = \"next\"\ntarget = [\"v0.2.x\"]\n+++\n\nGood.\n",
    )
    .unwrap();
    std::fs::write(features.join("f-bad.md"), "no fence here\n").unwrap();

    let tmp_md = unique_tmp("schema-err-md");
    std::fs::create_dir_all(&tmp_md).unwrap();
    let roadmap_md = tmp_md.join("ROADMAP.md");
    std::fs::write(&roadmap_md, "<a id=\"f-good\"></a>\n").unwrap();

    let report = roadmap_cli::validate::validate(&root, &roadmap_md).unwrap();
    assert_eq!(report.schema_errors.len(), 1, "{:?}", report.schema_errors);
    assert!(report.schema_errors[0].path.ends_with("f-bad.md"));
}

#[test]
fn anchor_collision_detected() {
    let root = unique_tmp("collision");
    let features = root.join("features");
    std::fs::create_dir_all(&features).unwrap();
    std::fs::write(root.join("config.toml"), "versions = [\"v0.2.x\"]\n").unwrap();
    // Two distinct IDs that lowercase to the same anchor.
    std::fs::write(
        features.join("f-foo-1.md"),
        "+++\nid = \"F-Foo\"\ntopic = \"X\"\nstatus = \"todo\"\npriority = \"next\"\ntarget = [\"v0.2.x\"]\n+++\n\nA.\n",
    )
    .unwrap();
    std::fs::write(
        features.join("f-foo-2.md"),
        "+++\nid = \"f-foo\"\ntopic = \"X\"\nstatus = \"todo\"\npriority = \"next\"\ntarget = [\"v0.2.x\"]\n+++\n\nB.\n",
    )
    .unwrap();

    let tmp_md = unique_tmp("collision-md");
    std::fs::create_dir_all(&tmp_md).unwrap();
    let roadmap_md = tmp_md.join("ROADMAP.md");
    std::fs::write(&roadmap_md, "").unwrap();

    let report = roadmap_cli::validate::validate(&root, &roadmap_md).unwrap();
    assert_eq!(report.anchor_collisions.len(), 1);
    assert_eq!(report.anchor_collisions[0].anchor, "f-foo");
}
