use criterion::{black_box, criterion_group, criterion_main, Criterion};
use combination_sum_ii::Solution;

fn benchmark_combination_sum_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("combination_sum_ii");
    
    // group.bench_function(format!("brute force"), |b| {
    //     b.iter(|| {
    //         let candidates = vec![1i32; 100];
    //         let target = 30i32;
    //         let result = Solution::combination_sum2_iterate_all(candidates, target);
    //         black_box(result);
    //     })
    // });

    group.bench_function(format!("optimized"), |b| {
        b.iter(|| {
            let candidates = vec![1i32; 100];
            let target = 30i32;
            let result = Solution::combination_sum2(candidates, target);
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_combination_sum_2);
criterion_main!(benches);
