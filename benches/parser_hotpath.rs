//! Micro-benchmarks for parser and engine hot paths.
//!
//! Covers the three layers the `dev-doc` CLI pass exercises most:
//!
//! - `split_sentences` — the tokenizer loop that runs once per paragraph.
//! - `parse_markdown` — the pulldown-cmark walk + section grouping.
//! - `Engine::lint_str` — the end-to-end path a user hits.
//!
//! Inputs come from the tracked `examples/public/` corpus so the numbers
//! reflect real prose, not synthetic input. Run with:
//!
//! ```sh
//! cargo bench --bench parser_hotpath
//! ```

#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use lucid_lint::parser::{parse_markdown, split_sentences};
use lucid_lint::types::SourceFile;
use lucid_lint::{Engine, Profile};

const EN_SHORT: &str =
    include_str!("../examples/public/en/good/simple-english-wikipedia/content.md");
const EN_LONG: &str = include_str!(
    "../examples/public/en/before-after/wcag-2-1-understanding-reading-level-3-1-5/content.md"
);

fn bench_split_sentences(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_sentences");
    group.throughput(Throughput::Bytes(EN_LONG.len() as u64));
    group.bench_function("en_long", |b| {
        b.iter(|| split_sentences(black_box(EN_LONG), 1, 1));
    });
    group.throughput(Throughput::Bytes(EN_SHORT.len() as u64));
    group.bench_function("en_short", |b| {
        b.iter(|| split_sentences(black_box(EN_SHORT), 1, 1));
    });
    group.finish();
}

fn bench_parse_markdown(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_markdown");
    group.throughput(Throughput::Bytes(EN_LONG.len() as u64));
    group.bench_function("en_long", |b| {
        b.iter(|| parse_markdown(black_box(EN_LONG), SourceFile::Anonymous));
    });
    group.finish();
}

fn bench_engine_lint(c: &mut Criterion) {
    let engine = Engine::with_profile(Profile::DevDoc);
    let mut group = c.benchmark_group("engine_lint_str");
    group.throughput(Throughput::Bytes(EN_LONG.len() as u64));
    group.bench_function("en_long_devdoc", |b| {
        b.iter(|| engine.lint_str(black_box(EN_LONG)));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_split_sentences,
    bench_parse_markdown,
    bench_engine_lint
);
criterion_main!(benches);
