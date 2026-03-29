use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Baseline benchmarks established in Iteration 1.
/// All subsequent iterations must not regress these numbers by more than 10%.

fn bench_format_detection(c: &mut Criterion) {
    let unified_diff = include_str!("../tests/fixtures/basic/single_hunk.diff");
    let search_replace = "<<<SEARCH\nfn foo() {}\n===\nfn foo() { bar() }\n>>>REPLACE";
    let full_rewrite = "fn hello() {\n    println!(\"hello, world\");\n}\n";

    let mut group = c.benchmark_group("format_detection");
    group.bench_function("unified_diff", |b| {
        b.iter(|| llm_apply::apply::formats::detect(black_box(unified_diff)))
    });
    group.bench_function("search_replace", |b| {
        b.iter(|| llm_apply::apply::formats::detect(black_box(search_replace)))
    });
    group.bench_function("full_rewrite", |b| {
        b.iter(|| llm_apply::apply::formats::detect(black_box(full_rewrite)))
    });
    group.finish();
}

fn bench_apply_by_file_size(c: &mut Criterion) {
    // These benchmarks are filled in as apply logic is implemented.
    // Targets from PLAN.md:
    //   p50 < 50ms for 500 LOC files
    //   p99 < 200ms for 500 LOC files
    let mut group = c.benchmark_group("apply");
    for loc in [100, 500, 2000] {
        group.bench_with_input(
            BenchmarkId::new("unified_diff", format!("{}_loc", loc)),
            &loc,
            |b, &_loc| {
                b.iter(|| {
                    // TODO: implement when apply logic exists (iteration 1)
                    black_box(())
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_format_detection, bench_apply_by_file_size);
criterion_main!(benches);
