pub struct Solution;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|v| v.1);

        let mut answer = vec![-1; queries.len()];
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut pq = BinaryHeap::new();
        let mut ii = 0;
        for (i, q) in &queries {
            while ii < intervals.len() && intervals[ii][0] <= *q {
                let intr = &intervals[ii];
                let len = intr[1] - intr[0] + 1;
                pq.push(Reverse((len, intr[1])));
                ii += 1;
            }

            while pq.peek().is_some_and(|Reverse((_, end))| end < q) {
                pq.pop();
            }

            if let Some(Reverse((len, _))) = pq.peek() {
                answer[*i] = *len;
            }
        }

        answer
    }
}
