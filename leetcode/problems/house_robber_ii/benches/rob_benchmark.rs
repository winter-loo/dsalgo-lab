use criterion::{black_box, criterion_group, criterion_main, Criterion};
use house_robber_ii::Solution;

fn bench_rob(c: &mut Criterion) {
    // Create a benchmark group to compare implementations
    let mut group = c.benchmark_group("House Robber II");
    
    // Small test case
    let small_case = vec![2, 3, 2];
    group.bench_function("refactored - small case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_refactored(black_box(small_case.clone())))
    });
    group.bench_function("initial - small case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_initial(black_box(small_case.clone())))
    });
    
    // Medium test case
    let medium_case = vec![1, 2, 3, 1, 5, 6, 7, 8, 2, 3];
    group.bench_function("refactored - medium case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_refactored(black_box(medium_case.clone())))
    });
    group.bench_function("initial - medium case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_initial(black_box(medium_case.clone())))
    });
    
    // Large test case
    let large_case = vec![104,209,137,52,158,67,213,86,141,110,151,127,238,147,169,138,240,185,246,225,147,203,83,83,131,227,54,78,165,180,214,151,111,161,233,147,124,143];
    group.bench_function("refactored - large case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_refactored(black_box(large_case.clone())))
    });
    group.bench_function("initial - large case", |b| {
        b.iter(|| Solution::rob_with_decision_tree_initial(black_box(large_case.clone())))
    });
    
    group.finish();
}

criterion_group!(benches, bench_rob);
criterion_main!(benches);
