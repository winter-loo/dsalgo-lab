pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut edges = BinaryHeap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let d = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push(Reverse((d, i, j)));
            }
        }
        fn find(roots: &mut Vec<usize>, p1: usize) -> usize {
            if roots[p1] != p1 {
                roots[p1] = find(roots, roots[p1]);
            }
            roots[p1]
        }
        // println!("edges={edges:?}");
        let mut roots: Vec<_> = (0..points.len()).collect();
        // println!("roots={roots:?}");
        let mut min_cost = 0;
        while let Some(Reverse((cost, p1, p2))) = edges.pop() {
            let (p1, p2) = (p1 as usize, p2 as usize);
            let r1 = find(&mut roots, p1);
            let r2 = find(&mut roots, p2);
            if r1 != r2 {
                // no cycle found
                roots[r1] = r2;
                min_cost += cost;
            }
        }
        min_cost
    }
}
