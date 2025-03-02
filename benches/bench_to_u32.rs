use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dsalgo::school::recursion::reinforcement::{to_u32_from_right, to_u32_from_left};

fn benchmark_to_u32(c: &mut Criterion) {
    c.bench_function("to_u32_from_right", |b| {
        b.iter(|| to_u32_from_right(black_box("1234567890")))
    });
}

fn benchmark_to_u32_2(c: &mut Criterion) {
    c.bench_function("to_u32_from_left", |b| {
        b.iter(|| to_u32_from_left(black_box("1234567890")))
    });
}

criterion_group!(benches, benchmark_to_u32, benchmark_to_u32_2);
criterion_main!(benches);
