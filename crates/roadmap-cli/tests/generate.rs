//! Integration test: load fixture .roadmap/, generate, snapshot.

use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn minimal_fixture_round_trip() {
    let root = fixture("minimal");
    let config = roadmap_cli::load_config(&root).unwrap();
    let mut features = roadmap_cli::load_features(&root).unwrap();
    roadmap_cli::sort_features(&mut features, &config);
    let out = roadmap_cli::render(&features);
    insta::assert_snapshot!(out);
}

#[test]
fn determinism_round_trip() {
    let root = fixture("minimal");
    let config = roadmap_cli::load_config(&root).unwrap();
    let mut a = roadmap_cli::load_features(&root).unwrap();
    let mut b = roadmap_cli::load_features(&root).unwrap();
    roadmap_cli::sort_features(&mut a, &config);
    roadmap_cli::sort_features(&mut b, &config);
    assert_eq!(roadmap_cli::render(&a), roadmap_cli::render(&b));
}
