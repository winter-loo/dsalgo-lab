use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reorder_list::{ListNode, Solution};

fn create_list(n: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut current = &mut head;
    for i in 2..=n {
        let node = Some(Box::new(ListNode::new(i)));
        current.as_mut().unwrap().next = node;
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

fn benchmark_reorder_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reorder List");
    
    // Test with different list sizes
    for size in [10, 100, 1000].iter() {
        let list = create_list(*size);
        
        group.bench_function(format!("deque method - size {}", size), |b| {
            b.iter(|| {
                let mut list = list.clone();
                Solution::reorder_list_deque(&mut list);
                black_box(list);
            })
        });

        group.bench_function(format!("two pointer method - size {}", size), |b| {
            b.iter(|| {
                let mut list = list.clone();
                Solution::reorder_list_two_pointer(&mut list);
                black_box(list);
            })
        });

        group.bench_function(format!("counting method - size {}", size), |b| {
            b.iter(|| {
                let mut list = list.clone();
                Solution::reorder_list_couting(&mut list);
                black_box(list);
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_reorder_list);
criterion_main!(benches); 